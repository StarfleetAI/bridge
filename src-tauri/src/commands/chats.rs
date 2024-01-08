// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// TODO(ri-nat): I don't really know, why Clippy is mad about these here, but let make him quiet for now.
#![allow(clippy::used_underscore_binding)]

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use tauri::State;

use crate::{
    commands::{
        agents::AgentRow,
        messages::{Role, Status},
    },
    types::{DbMutex, Result},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatsList {
    pub chats: Vec<Chat>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChat {
    pub agent_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteChat {
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetChat {
    pub id: i64,
}

/// List all chats.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_chats(pool_mutex: State<'_, DbMutex>) -> Result<ChatsList> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let chats = query_as!(
        Chat,
        "SELECT id, title, created_at, updated_at FROM chats ORDER BY id DESC"
    )
    .fetch_all(pool)
    .await
    .with_context(|| "Failed to fetch chats")?;

    Ok(ChatsList { chats })
}

/// Get chat by id.
///
/// # Errors
///
/// Returns error if chat with given id does not exist.
#[tauri::command]
pub async fn get_chat(request: GetChat, pool_mutex: State<'_, DbMutex>) -> Result<Chat> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let chat = query_as!(
        Chat,
        "SELECT id, title, created_at, updated_at FROM chats WHERE id = $1",
        request.id
    )
    .fetch_one(pool)
    .await
    .with_context(|| "Failed to fetch chat")?;

    Ok(chat)
}

/// Create new chat with agent.
///
/// # Errors
///
/// Returns error if there was a problem while inserting new chat.
#[tauri::command]
pub async fn create_chat(request: CreateChat, pool_mutex: State<'_, DbMutex>) -> Result<Chat> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let agent = query_as!(
        AgentRow,
        "SELECT * FROM agents WHERE id = $1",
        request.agent_id,
    )
    .fetch_one(&mut *tx)
    .await
    .with_context(|| "Failed to fetch agent")?;

    let now = Utc::now();
    let chat = query_as!(
        Chat,
        r#"
        INSERT INTO chats (created_at, updated_at)
        VALUES ($1, $1)
        RETURNING id, title, created_at, updated_at
        "#,
        now
    )
    .fetch_one(&mut *tx)
    .await
    .with_context(|| "Failed to create chat")?;

    // Add agent to chat.
    query!(
        "INSERT INTO agents_chats (agent_id, chat_id) VALUES ($1, $2)",
        request.agent_id,
        chat.id
    )
    .execute(&mut *tx)
    .await
    .with_context(|| "Failed to add agent to chat")?;

    // Insert system prompt message to chat.
    query!(
        "INSERT INTO messages (chat_id, status, role, content, created_at) VALUES ($1, $2, $3, $4, $5)",
        chat.id,
        Status::Completed,
        Role::System,
        agent.system_message,
        now
    )
    .execute(&mut *tx)
    .await
    .with_context(|| "Failed to insert system prompt message")?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(chat)
}

/// Delete chat by id.
///
/// # Errors
///
/// Returns error if chat with given id does not exist.
#[tauri::command]
pub async fn delete_chat(request: DeleteChat, pool_mutex: State<'_, DbMutex>) -> Result<()> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    query!("DELETE FROM agents_chats WHERE chat_id = $1", request.id)
        .execute(&mut *tx)
        .await
        .with_context(|| "Failed to delete agents from chat")?;

    query!("DELETE FROM chats WHERE id = $1", request.id)
        .execute(&mut *tx)
        .await
        .with_context(|| "Failed to delete chat")?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}
