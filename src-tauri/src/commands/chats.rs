// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// TODO(ri-nat): I don't really know, why Clippy is mad about these here, but let make him quiet for now.
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
pub async fn list_chats(pool: State<'_, DbPool>) -> Result<ChatsList> {
    let chats = repo::chats::list(&*pool).await?;

    Ok(ChatsList { chats })
}

/// Get chat by id.
///
/// # Errors
///
/// Returns error if chat with given id does not exist.
#[tauri::command]
pub async fn get_chat(request: GetChat, pool: State<'_, DbPool>) -> Result<Chat> {
    repo::chats::get(&*pool, request.id).await
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
pub async fn delete_chat(request: DeleteChat, pool: State<'_, DbPool>) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    repo::agents_chats::delete_for_chat(&mut *tx, request.id).await?;
    repo::chats::delete(&mut *tx, request.id).await?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}
