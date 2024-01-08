// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// TODO(ri-nat): I don't really know, why Clippy is mad about these here, but let make him quiet for now.
#![allow(clippy::used_underscore_binding)]

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use log::debug;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use tauri::{Manager, State, Window};
use tokio::sync::RwLock;

use crate::{
    clients::openai::{Client, CreateChatCompletionRequest, Tool},
    commands::agents::AgentRow,
    errors,
    settings::Settings,
    types::{DbMutex, Result},
};

use super::abilities::Ability;

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

impl From<String> for Role {
    fn from(role: String) -> Self {
        match role.as_str() {
            "System" => Role::System,
            "Assistant" => Role::Assistant,
            "Tool" => Role::Tool,
            _ => Role::User,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
pub enum Status {
    Writing,
    WaitingForToolCall,
    Completed,
}

impl From<String> for Status {
    fn from(status: String) -> Self {
        match status.as_str() {
            "Writing" => Status::Writing,
            "WaitingForToolCall" => Status::WaitingForToolCall,
            _ => Status::Completed,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub agent_id: Option<i64>,
    pub status: Status,
    pub role: Role,
    pub content: Option<String>,
    pub prompt_tokens: Option<i64>,
    pub completion_tokens: Option<i64>,
    pub tool_calls: Option<String>,
    pub tool_call_id: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct ListMessages {
    pub chat_id: i64,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MessagesList {
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMessage {
    pub chat_id: i64,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteMessage {
    pub id: i64,
}

/// List all messages.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_messages(
    request: ListMessages,
    pool_mutex: State<'_, DbMutex>,
) -> Result<MessagesList> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let messages = query_as!(
        Message,
        r#"
        SELECT
            id as "id!", chat_id, status, agent_id, role, content, prompt_tokens,
            completion_tokens, tool_calls, tool_call_id, created_at
        FROM messages
        WHERE chat_id = $1
        ORDER BY id ASC
        "#,
        request.chat_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| "Failed to fetch messages")?;

    Ok(MessagesList { messages })
}

/// Create new message.
///
/// # Errors
///
/// Returns error if there was a problem while inserting new message.
///
/// # Panics
///
/// Panics if there is an error when converting message from a database row to a API-compatible
/// message. Should never happen.
// TODO: refactor this function.
#[allow(clippy::too_many_lines)]
#[tauri::command]
pub async fn create_message(
    window: Window,
    request: CreateMessage,
    pool_mutex: State<'_, DbMutex>,
    settings: State<'_, RwLock<Settings>>,
) -> Result<()> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;
    let settings_guard = settings.read().await;

    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let now = Utc::now();
    let message = query_as!(
        Message,
        "INSERT INTO messages (
            chat_id, status, role, content, created_at
        ) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        request.chat_id,
        Status::Completed,
        Role::User,
        request.text,
        now
    )
    .fetch_one(&mut *tx)
    .await
    .with_context(|| "Failed to create message")?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    window
        .emit_all("messages:created", &message)
        .with_context(|| "Failed to emit event")?;

    tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let messages: Vec<Message> = query_as!(
        Message,
        r#"
        SELECT
            id as "id!", chat_id, agent_id, status, role, content, prompt_tokens,
            completion_tokens, tool_calls, tool_call_id, created_at
        FROM messages
        WHERE chat_id = $1
        ORDER BY id ASC
        "#,
        message.chat_id
    )
    .fetch_all(&mut *tx)
    .await
    .with_context(|| "Failed to fetch chat messages")?;

    debug!("Messages so far: {:?}", messages);

    // Get current agent.
    let agent = query_as!(
        AgentRow,
        r#"
        SELECT agents.*
        FROM agents
        INNER JOIN agents_chats ON agents.id = agents_chats.agent_id
        WHERE agents_chats.chat_id = $1
        "#,
        request.chat_id,
    )
    .fetch_one(&mut *tx)
    .await
    .with_context(|| "Failed to fetch agent")?;

    // TODO: correctly handle errors here
    let req_messages = messages
        .into_iter()
        .map(|message| crate::clients::openai::Message::try_from(message).unwrap())
        .collect();

    // Insert dummy message to chat.
    let message = query_as!(
        Message,
        r#"
        INSERT INTO messages (
            chat_id, agent_id, status, role, created_at
        ) VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        request.chat_id,
        agent.id,
        Status::Writing,
        Role::Assistant,
        now
    )
    .fetch_one(&mut *tx)
    .await
    .with_context(|| "Failed to insert dummy assistant message")?;

    window
        .emit_all("messages:created", &message)
        .with_context(|| "Failed to emit event")?;

    // Send request to OpenAI.
    let client = Client::new(
        settings_guard
            .openai_api_key
            .as_ref()
            .with_context(|| "Failed to get openai api key")?,
    );

    let abilities: Vec<Ability> = query_as!(
        Ability,
        r#"
        SELECT
            abilities.id as "id!", abilities.name, abilities.description, abilities.code,
            abilities.created_at, abilities.updated_at, abilities.parameters_json
        FROM abilities
        INNER JOIN agent_abilities ON abilities.id = agent_abilities.ability_id
        WHERE agent_abilities.agent_id = $1
        "#,
        agent.id,
    )
    .fetch_all(&mut *tx)
    .await
    .with_context(|| "Failed to fetch agent abilities")?;

    let tools: Vec<Tool> = abilities
        .into_iter()
        .map(
            |ability| match serde_json::from_str(&ability.parameters_json) {
                Ok(function) => Ok(Tool {
                    type_: "function".to_string(),
                    function,
                }),
                Err(err) => Err(errors::Error::Internal(err.into())),
            },
        )
        .collect::<Result<Vec<Tool>>>()?;

    debug!("Tools: {:?}", tools);

    let completion = client
        .create_chat_completion(CreateChatCompletionRequest {
            model: "gpt-4".to_string(),
            messages: req_messages,
            tools,
        })
        .await
        .with_context(|| "Failed to create chat completion")?;

    // Update message in chat.
    //
    // We're only using the first message for now.
    let message = match &completion.choices[0].message {
        crate::clients::openai::Message::Assistant {
            content,
            tool_calls,
            ..
        } => {
            let mut status = Status::Completed;
            let tool_calls = match &tool_calls {
                Some(calls) => {
                    status = Status::WaitingForToolCall;

                    Some(
                        serde_json::to_string(&calls)
                            .with_context(|| "Failed to serialize tool calls")?,
                    )
                }
                None => None,
            };

            query_as!(
                Message,
                r#"
                UPDATE messages
                SET
                    status = $2,
                    content = $3,
                    prompt_tokens = $4,
                    completion_tokens = $5,
                    tool_calls = $6
                WHERE id = $1
                RETURNING
                    id as "id!", chat_id, agent_id, status, role, content, prompt_tokens,
                    completion_tokens, tool_calls, tool_call_id, created_at
                "#,
                message.id,
                status,
                content,
                completion.usage.prompt_tokens,
                completion.usage.completion_tokens,
                tool_calls,
            )
            .fetch_one(&mut *tx)
            .await
            .with_context(|| "Failed to update assistant message")?
        }
        _ => return Err(anyhow::anyhow!("Unexpected message type").into()),
    };

    window
        .emit_all("messages:updated", &message)
        .with_context(|| "Failed to emit event")?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}

/// Delete message by id.
///
/// # Errors
///
/// Returns error if there was a problem while deleting message.
#[tauri::command]
pub async fn delete_message(request: DeleteMessage, pool_mutex: State<'_, DbMutex>) -> Result<()> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    query!("DELETE FROM messages WHERE id = $1", request.id)
        .execute(&mut *tx)
        .await
        .with_context(|| "Failed to delete message")?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}
