// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Executor, Sqlite};

use crate::types::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// List all chats.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn list<'a, E>(executor: E) -> Result<Vec<Chat>>
where
    E: Executor<'a, Database = Sqlite>,
{
    Ok(query_as!(
        Chat,
        "SELECT id, title, created_at, updated_at FROM chats ORDER BY id DESC"
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
    Ok(query_as!(
        Chat,
        "SELECT id, title, created_at, updated_at FROM chats WHERE id = $1 LIMIT 1",
        id
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to fetch chat")?)
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
pub async fn create<'a, E>(executor: E) -> Result<Chat>
where
    E: Executor<'a, Database = Sqlite>,
{
    let now = Utc::now();
    Ok(query_as!(
        Chat,
        "INSERT INTO chats (created_at, updated_at) VALUES ($1, $1) RETURNING *",
        now
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to create chat")?)
}
