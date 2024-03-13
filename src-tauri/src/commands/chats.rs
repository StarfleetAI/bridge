// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    repo::{
        self,
        chats::Chat,
        messages::{Role, Status},
    },
    types::{DbPool, Result},
};

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatsList {
    pub chats: Vec<Chat>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChat {
    pub agent_id: i64,
}

/// List all chats.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_chats(pool: State<'_, DbPool>, is_pinned: Option<bool>) -> Result<ChatsList> {
    let chats = repo::chats::list(&*pool, is_pinned).await?;

    Ok(ChatsList { chats })
}

/// Get chat by id.
///
/// # Errors
///
/// Returns error if chat with given id does not exist.
#[tauri::command]
pub async fn get_chat(id: i64, pool: State<'_, DbPool>) -> Result<Chat> {
    repo::chats::get(&*pool, id).await
}

/// Create new chat with agent.
///
/// # Errors
///
/// Returns error if there was a problem while inserting new chat.
#[tauri::command]
pub async fn create_chat(request: CreateChat, pool: State<'_, DbPool>) -> Result<Chat> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let agent = repo::agents::get(&mut *tx, request.agent_id).await?;
    let chat = repo::chats::create(&mut *tx).await?;

    // Add agent to chat
    repo::agents_chats::create(&mut *tx, request.agent_id, chat.id).await?;

    // Insert system prompt message to chat
    repo::messages::create(
        &mut *tx,
        repo::messages::CreateParams {
            chat_id: chat.id,
            status: Status::Completed,
            role: Role::System,
            content: Some(agent.system_message),
            ..Default::default()
        },
    )
    .await?;

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
pub async fn delete_chat(id: i64, pool: State<'_, DbPool>) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    repo::tasks::delete_for_chat(&mut *tx, id).await?;
    repo::messages::delete_for_chat(&mut *tx, id).await?;
    repo::agents_chats::delete_for_chat(&mut *tx, id).await?;
    repo::chats::delete(&mut *tx, id).await?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}

/// Update chat title by id.
///
/// # Errors
///
/// Returns error if there was a problem while updating the chat title or if the chat with the given ID does not exist.
#[tauri::command]
pub async fn update_chat_title(id: i64, title: String, pool: State<'_, DbPool>) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    repo::chats::update_title(&mut *tx, id, &title).await?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}

/// Toggle chat is pinned status by id.
///
/// # Errors
///
/// Returns error if the chat with the given ID does not exist.
#[tauri::command]
pub async fn toggle_chat_is_pinned(id: i64, pool: State<'_, DbPool>) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    repo::chats::toggle_is_pinned(&mut *tx, id).await?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}

/// Change chat model full name by id
///
/// # Errors
///
/// Return error if the chat with the given ID does not exist.
#[tauri::command]
pub async fn update_chat_model_full_name(
    id: i64,
    model_full_name: String,
    pool: State<'_, DbPool>,
) -> Result<()> {
    // Acquire a connection from the pool
    let mut conn = pool
        .inner()
        .acquire()
        .await
        .with_context(|| "Failed to acquire a connection from the pool")?;

    // Execute the update operation directly without starting a transaction
    repo::chats::update_model_full_name(&mut *conn, id, &model_full_name).await?;

    Ok(())
}
