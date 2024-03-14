// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use markdown::to_html;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use sqlx::{query, query_as, query_scalar, Executor, Sqlite, SqliteConnection};

use crate::messages::Error;
use crate::types::Result;

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Default, PartialEq, Clone)]
pub enum Role {
    #[default]
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

#[derive(Serialize, Deserialize, Debug, sqlx::Type, PartialEq, Default, Clone, Copy)]
pub enum Status {
    #[default]
    Writing,
    WaitingForToolCall,
    Completed,
    Failed,
    ToolCallDenied,
}

impl From<String> for Status {
    fn from(status: String) -> Self {
        match status.as_str() {
            "Writing" => Status::Writing,
            "WaitingForToolCall" => Status::WaitingForToolCall,
            "Failed" => Status::Failed,
            "ToolCallDenied" => Status::ToolCallDenied,
            _ => Status::Completed,
        }
    }
}

/// Safely render markdown in a message as an untrusted user input.
fn serialize_content<S>(
    content: &Option<String>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&to_html(content.as_ref().unwrap_or(&String::new())))
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub agent_id: Option<i64>,
    pub status: Status,
    pub role: Role,
    #[serde(serialize_with = "serialize_content")]
    pub content: Option<String>,
    pub prompt_tokens: Option<i64>,
    pub completion_tokens: Option<i64>,
    pub tool_calls: Option<String>,
    pub tool_call_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreateParams {
    pub chat_id: i64,
    pub agent_id: Option<i64>,
    pub status: Status,
    pub role: Role,
    pub content: Option<String>,
    pub prompt_tokens: Option<i64>,
    pub completion_tokens: Option<i64>,
    pub tool_calls: Option<String>,
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Default)]
pub struct ListParams {
    pub chat_id: i64,
}

#[derive(Debug, Default)]
pub struct UpdateWithCompletionResultParams {
    pub id: i64,
    pub status: Status,
    pub content: Option<String>,
    pub prompt_tokens: Option<i64>,
    pub completion_tokens: Option<i64>,
    pub tool_calls: Option<String>,
}

/// List all messages.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn list<'a, E>(executor: E, params: ListParams) -> Result<Vec<Message>>
where
    E: Executor<'a, Database = Sqlite>,
{
    let messages = sqlx::query_as!(
        Message,
        r#"
        SELECT
            id as "id!", chat_id, status, agent_id, role, content, prompt_tokens,
            completion_tokens, tool_calls, tool_call_id, created_at, updated_at
        FROM messages
        WHERE chat_id = $1
        ORDER BY id ASC
        "#,
        params.chat_id,
    )
    .fetch_all(executor)
    .await
    .with_context(|| "Failed to list messages")?;

    Ok(messages)
}

/// Create message.
///
/// # Errors
///
/// Returns error if there was a problem while creating message.
pub async fn create<'a, E>(executor: E, params: CreateParams) -> Result<Message>
where
    E: Executor<'a, Database = Sqlite>,
{
    let now = Utc::now();
    let message = sqlx::query_as!(
        Message,
        r#"
        INSERT INTO messages (chat_id, agent_id, status, role, content, prompt_tokens,
            completion_tokens, tool_calls, tool_call_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $10)
        RETURNING id as "id!", chat_id, status, agent_id, role, content, prompt_tokens,
            completion_tokens, tool_calls, tool_call_id, created_at, updated_at
        "#,
        params.chat_id,
        params.agent_id,
        params.status,
        params.role,
        params.content,
        params.prompt_tokens,
        params.completion_tokens,
        params.tool_calls,
        params.tool_call_id,
        now,
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to create message")?;

    Ok(message)
}

/// Get message by id.
///
/// # Errors
///
/// Returns error if there was a problem while fetching message.
pub async fn get<'a, E>(executor: E, id: i64) -> Result<Message>
where
    E: Executor<'a, Database = Sqlite>,
{
    let message = sqlx::query_as!(
        Message,
        r#"
        SELECT
            id as "id!", chat_id, status, agent_id, role, content, prompt_tokens,
            completion_tokens, tool_calls, tool_call_id, created_at, updated_at
        FROM messages
        WHERE id = $1
        "#,
        id,
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to get message")?;

    Ok(message)
}

/// Get last message id.
///
/// # Errors
///
/// Returns error if there was a problem while fetching last message id.
pub async fn get_last_message_id<'a, E>(executor: E, chat_id: i64) -> Result<Option<i64>>
where
    E: Executor<'a, Database = Sqlite>,
{
    Ok(query_scalar!(
        "SELECT CAST(MAX(id) AS INTEGER) FROM messages WHERE chat_id = $1",
        chat_id
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to fetch last message id")?)
}

/// Get last message for chat.
///
/// # Errors
///
/// Returns error if there was a problem while fetching last message.
pub async fn get_last_message<'a, E>(executor: E, chat_id: i64) -> Result<Option<Message>>
where
    E: Executor<'a, Database = Sqlite>,
{
    Ok(query_as!(
        Message,
        r#"
        SELECT
            id as "id!", chat_id, status, agent_id, role, content, prompt_tokens,
            completion_tokens, tool_calls, tool_call_id, created_at, updated_at
        FROM messages
        WHERE chat_id = $1
        ORDER BY id DESC
        LIMIT 1
        "#,
        chat_id,
    )
    .fetch_optional(executor)
    .await
    .with_context(|| "Failed to get last message")?)
}

/// Update message status.
///
/// # Errors
///
/// Returns error if there was a problem while updating message status.
pub async fn update_status<'a, E>(executor: E, id: i64, status: Status) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    query!("UPDATE messages SET status = $1 WHERE id = $2", status, id)
        .execute(executor)
        .await
        .with_context(|| "Failed to update message status")?;

    Ok(())
}

/// Update assistant message with completion result.
///
/// # Errors
///
/// Returns error if there was a problem while updating assistant message.
pub async fn update_with_completion_result<'a, E>(
    executor: E,
    params: UpdateWithCompletionResultParams,
) -> Result<Message>
where
    E: Executor<'a, Database = Sqlite>,
{
    let now = Utc::now();
    let message = query_as!(
        Message,
        r#"
        UPDATE messages
        SET
            status = $2,
            content = $3,
            prompt_tokens = $4,
            completion_tokens = $5,
            tool_calls = $6,
            updated_at = $7
        WHERE id = $1
        RETURNING
            id as "id!", chat_id, agent_id, status, role, content, prompt_tokens,
            completion_tokens, tool_calls, tool_call_id, created_at, updated_at
        "#,
        params.id,
        params.status,
        params.content,
        params.prompt_tokens,
        params.completion_tokens,
        params.tool_calls,
        now
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to update assistant message")?;

    Ok(message)
}

/// Delete message.
///
/// # Errors
///
/// Returns error if there was a problem while deleting message.
pub async fn delete<'a, E>(executor: E, id: i64) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    query!("DELETE FROM messages WHERE id = $1", id)
        .execute(executor)
        .await
        .with_context(|| "Failed to delete message")?;

    Ok(())
}

/// Update message content.
///
/// # Errors
///
/// Returns error if there was a problem while updating message content.
pub async fn update_message_content<'a, E>(executor: E, id: i64, content: &str) -> Result<Message>
where
    E: Executor<'a, Database = Sqlite>,
{
    let now = Utc::now();
    let message = query_as!(
        Message,
        r#"
        UPDATE messages
        SET content = $2, updated_at = $3
        WHERE id = $1
        RETURNING
            id as "id!", chat_id, agent_id, status, role, content, prompt_tokens,
            completion_tokens, tool_calls, tool_call_id, created_at, updated_at
        "#,
        id,
        content,
        now
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to update message content")?;

    Ok(message)
}

/// Transitions messages from one status to another.
///
/// # Errors
///
/// Returns error if there was a problem while updating messages.
pub async fn transition_all<'a, E>(executor: E, from: Status, to: Status) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    let now = Utc::now();
    query!(
        "UPDATE messages
         SET status = $1, updated_at = $3
         WHERE status = $2",
        to,
        from,
        now
    )
    .execute(executor)
    .await
    .with_context(|| "Failed to set `{from}` messages to `{to}`")?;

    Ok(())
}

/// Delete messages for chat.
///
/// # Errors
///
/// Returns error if there was a problem while deleting messages.
pub async fn delete_for_chat<'a, E>(executor: E, chat_id: i64) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    query!("DELETE FROM messages WHERE chat_id = $1", chat_id)
        .execute(executor)
        .await
        .with_context(|| "Failed to delete messages for chat")?;

    Ok(())
}

/// Create tool call denied
///
/// # Errors
///
/// Returns error if there was a problem while creating message.
pub async fn create_tool_call_denied(
    conn: &mut SqliteConnection,
    message: &Message,
) -> Result<Vec<Message>> {
    match &message.tool_calls {
        Some(tool_calls) => {
            let tool_calls: Vec<Value> =
                serde_json::from_str(tool_calls).with_context(|| "Failed to parse tool calls")?;

            let mut messages = Vec::with_capacity(tool_calls.len());
            for tool_call in &tool_calls {
                let tool_call = tool_call.as_object().ok_or(Error::NoToolCallsFound)?;
                let tool_call_id = tool_call["id"].as_str().ok_or(Error::NoToolCallId)?;

                messages.push(
                    create(
                        &mut *conn,
                        CreateParams {
                            chat_id: message.chat_id,
                            status: Status::ToolCallDenied,
                            role: Role::Tool,
                            content: Some("Tool call denied".to_string()),
                            tool_call_id: Some(tool_call_id.to_string()),

                            ..Default::default()
                        },
                    )
                    .await?,
                );
            }

            Ok(messages)
        }
        None => Err(Error::NoToolCallsFound.into()),
    }
}
