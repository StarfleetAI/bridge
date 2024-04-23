// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use anyhow::Context;
use bridge_common::{repo, types::chats::Chat};
use serde::{Deserialize, Serialize};
use tauri::State;
use tracing::error;

use crate::types::{DbPool, Result};

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatsList {
    pub chats: Vec<Chat>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChat {
    pub agent_id: i32,
}

/// List all chats.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_chats(pool: State<'_, DbPool>, is_pinned: Option<bool>) -> Result<ChatsList> {
    let chats = repo::chats::list(&*pool, crate::CID, is_pinned).await?;

    Ok(ChatsList { chats })
}

/// Get chat by id.
///
/// # Errors
///
/// Returns error if chat with given id does not exist.
#[tauri::command]
pub async fn get_chat(id: i32, pool: State<'_, DbPool>) -> Result<Chat> {
    Ok(repo::chats::get(&*pool, crate::CID, id).await?)
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

    let agent = repo::agents::get(&mut *tx, crate::CID, request.agent_id).await?;
    let chat = repo::chats::create(
        &mut *tx,
        crate::CID,
        bridge_common::types::chats::Kind::Direct,
    )
    .await?;

    // Add agent to chat
    repo::agents_chats::create(&mut *tx, crate::CID, request.agent_id, chat.id).await?;

    // Insert system prompt message to chat
    repo::messages::create(
        &mut *tx,
        crate::CID,
        repo::messages::CreateParams {
            chat_id: chat.id,
            status: bridge_common::types::messages::Status::Completed,
            role: bridge_common::types::messages::Role::System,
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
pub async fn delete_chat(id: i32, pool: State<'_, DbPool>) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    repo::tasks::delete_for_chat(&mut *tx, crate::CID, id).await?;
    repo::messages::delete_for_chat(&mut *tx, crate::CID, id).await?;
    repo::agents_chats::delete_for_chat(&mut *tx, crate::CID, id).await?;
    repo::chats::delete(&mut *tx, crate::CID, id).await?;

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
pub async fn update_chat_title(id: i32, title: String, pool: State<'_, DbPool>) -> Result<()> {
    Ok(repo::chats::update_title(&*pool, crate::CID, id, &title).await?)
}

/// Toggle chat is pinned status by id.
///
/// # Errors
///
/// Returns error if the chat with the given ID does not exist.
#[tauri::command]
pub async fn toggle_chat_is_pinned(id: i32, pool: State<'_, DbPool>) -> Result<()> {
    Ok(repo::chats::toggle_is_pinned(&*pool, crate::CID, id).await?)
}

/// Change chat model full name by id
///
/// # Errors
///
/// Return error if the chat with the given ID does not exist.
#[tauri::command]
pub async fn update_chat_model_full_name(
    id: i32,
    model_full_name: Option<String>,
    pool: State<'_, DbPool>,
) -> Result<()> {
    let maybe_model_id = match model_full_name {
        Some(model_full_name) => {
            if let Some(model) =
                repo::models::get_by_full_name(&*pool, crate::CID, &model_full_name).await?
            {
                Some(model.id)
            } else {
                error!(
                    "Model with full name {} does not exist in the database",
                    model_full_name
                );

                None
            }
        }
        None => None,
    };

    Ok(repo::chats::update_model_id(&*pool, crate::CID, id, maybe_model_id).await?)
}
