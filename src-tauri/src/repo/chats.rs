// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Executor, Sqlite};

use crate::types::Result;

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Default, PartialEq, Clone)]
pub enum Kind {
    #[default]
    Direct,
    Control,
    Execution,
}

impl From<String> for Kind {
    fn from(kind: String) -> Self {
        match kind.as_str() {
            "Control" => Kind::Control,
            "Execution" => Kind::Execution,
            _ => Kind::Direct,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    pub model_full_name: Option<String>,
    pub title: String,
    pub is_pinned: bool,
    pub kind: Kind,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// List all chats.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn list<'a, E>(executor: E, is_pinned: Option<bool>) -> Result<Vec<Chat>>
where
    E: Executor<'a, Database = Sqlite>,
{
    if let Some(is_pinned) = is_pinned {
        return Ok(query_as!(
            Chat,
            r#"
            SELECT
                id, model_full_name, title, created_at, updated_at, is_pinned, kind
            FROM chats
            WHERE
                is_pinned = $1 AND
                kind = $2
            ORDER BY id DESC
            "#,
            is_pinned,
            Kind::Direct,
        )
        .fetch_all(executor)
        .await
        .with_context(|| "Failed to fetch chats")?);
    }

    Ok(query_as!(
        Chat,
        "SELECT * FROM chats WHERE kind = $1 ORDER BY id DESC",
        Kind::Direct,
    )
    .fetch_all(executor)
    .await
    .with_context(|| "Failed to fetch chats")?)
}

/// Get chat by id.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn get<'a, E>(executor: E, id: i64) -> Result<Chat>
where
    E: Executor<'a, Database = Sqlite>,
{
    Ok(
        query_as!(Chat, "SELECT * FROM chats WHERE id = $1 LIMIT 1", id)
            .fetch_one(executor)
            .await
            .with_context(|| "Failed to fetch chat")?,
    )
}

/// Delete chat by id.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn delete<'a, E>(executor: E, id: i64) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    query!("DELETE FROM chats WHERE id = $1", id)
        .execute(executor)
        .await
        .with_context(|| "Failed to delete chat")?;

    Ok(())
}

/// Create chat.
///
/// # Errors
///
/// Returns error if there was a problem while creating chat.
pub async fn create<'a, E>(executor: E, kind: Kind) -> Result<Chat>
where
    E: Executor<'a, Database = Sqlite>,
{
    let now = Utc::now();
    Ok(query_as!(
        Chat,
        "INSERT INTO chats (kind, created_at, updated_at) VALUES ($1, $2, $2) RETURNING *",
        kind,
        now
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to create chat")?)
}

/// Update chat title by id.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database or if the chat with the given ID does not exist.
pub async fn update_title<'a, E>(executor: E, id: i64, title: &str) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    let now = Utc::now();
    query!(
        "UPDATE chats SET title = $1, updated_at = $2 WHERE id = $3",
        title,
        now,
        id
    )
    .execute(executor)
    .await
    .with_context(|| format!("Failed to update title for chat with id: {id}"))?;

    Ok(())
}

/// Toggle chat is pinned status by id.
///
/// # Errors
///
/// Returns error if the chat with the given ID does not exist.
pub async fn toggle_is_pinned<'a, E>(executor: E, id: i64) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    query!(
        "UPDATE chats SET is_pinned = NOT is_pinned WHERE id = $1",
        id
    )
    .execute(executor)
    .await
    .with_context(|| format!("Failed to toggle pin status for chat with id: {id}"))?;

    Ok(())
}

/// Change chat model full name by id
///
/// # Errors
///
/// Return error if the chat with the given ID does not exist.
pub async fn update_model_full_name<'a, E>(
    executor: E,
    id: i64,
    model_full_name: Option<String>,
) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    let now = Utc::now();
    query!(
        "UPDATE chats SET model_full_name = $1, updated_at = $3 WHERE id = $2",
        model_full_name,
        id,
        now
    )
    .execute(executor)
    .await
    .with_context(|| format!("Failed to change model for chat with id: {id}"))?;

    Ok(())
}
