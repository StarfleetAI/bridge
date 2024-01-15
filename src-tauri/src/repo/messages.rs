// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, query_scalar, Executor, Sqlite};

use crate::types::Result;

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Default)]
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
            completion_tokens, tool_calls, tool_call_id, created_at
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
            completion_tokens, tool_calls, tool_call_id, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id as "id!", chat_id, status, agent_id, role, content, prompt_tokens,
            completion_tokens, tool_calls, tool_call_id, created_at
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
            completion_tokens, tool_calls, tool_call_id, created_at
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
pub async fn get_last_message_id<'a, E>(executor: E, chat_id: i64) -> Result<i64>
where
    E: Executor<'a, Database = Sqlite>,
{
    Ok(query_scalar!(
        "SELECT CAST(MAX(id) AS INTEGER) FROM messages WHERE chat_id = $1",
        chat_id
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to fetch last message id")?
    .unwrap_or_default())
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
    let message = query_as!(
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
        params.id,
        params.status,
        params.content,
        params.prompt_tokens,
        params.completion_tokens,
        params.tool_calls,
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
