// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use anyhow::{anyhow, Context};
use bridge_common::channel::{Channel, Event};
use bridge_common::chats::CreateCompletionParams;
use bridge_common::repo;
use bridge_common::repo::messages::{CreateParams, ListParams};
use bridge_common::settings::Settings;
use bridge_common::types::messages::{Message, Role, Status};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};
use tokio::sync::RwLock;
use tracing::{debug, trace, warn};
use tracing::{error, instrument};

use crate::types::{DbPool, Result};

#[derive(Serialize, Deserialize, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct ListMessages {
    pub chat_id: i32,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MessagesList {
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMessage {
    pub chat_id: i32,
    pub text: String,
}

/// List all messages.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
#[instrument(skip(pool))]
pub async fn list_messages(request: ListMessages, pool: State<'_, DbPool>) -> Result<MessagesList> {
    debug!("Listing messages for chat");

    let messages = repo::messages::list(
        &*pool,
        crate::CID,
        ListParams {
            chat_id: request.chat_id,
        },
    )
    .await?;

    Ok(MessagesList { messages })
}

/// Create new message.
///
/// # Errors
///
/// Returns error if there was a problem while inserting new message.
#[tauri::command]
#[instrument(skip_all)]
pub async fn create_message(
    request: CreateMessage,
    channel: State<'_, Channel>,
    pool: State<'_, DbPool>,
    settings: State<'_, RwLock<Settings>>,
) -> Result<()> {
    debug!("Creating message");

    let mut tx = pool.begin().await.context("Failed to begin transaction")?;

    // Retrieve the last message for the chat
    let last_message_id =
        repo::messages::get_last_message_id(&mut *tx, crate::CID, request.chat_id)
            .await?
            .context("Failed to get last message id")?;
    let mut last_message = repo::messages::get(&mut *tx, crate::CID, last_message_id).await?;

    // If last message status is waiting for tool call, deny it
    if last_message.status == Status::WaitingForToolCall {
        // Update the message status to ToolCallDenied
        repo::messages::update_status(
            &mut *tx,
            crate::CID,
            last_message_id,
            Status::ToolCallDenied,
        )
        .await?;
        // Create a new message indicating the tool call was denied
        let denied_messages =
            repo::messages::create_tool_call_denied(&mut *tx, crate::CID, &last_message).await?;

        last_message.status = Status::ToolCallDenied;
        channel
            .emit(crate::UID, Event::MessageUpdated(&last_message))
            .await?;

        for denied_message in denied_messages {
            channel
                .emit(crate::UID, Event::MessageCreated(&denied_message))
                .await?;
        }
    }

    let message = repo::messages::create(
        &mut *tx,
        crate::CID,
        CreateParams {
            chat_id: request.chat_id,
            status: Status::Completed,
            role: Role::User,
            content: Some(request.text),

            ..Default::default()
        },
    )
    .await?;

    tx.commit().await.context("Failed to commit transaction")?;

    channel
        .emit(crate::UID, Event::MessageCreated(&message))
        .await?;

    let sett = settings.read().await.clone();

    let chat = repo::chats::get(&*pool, crate::CID, message.chat_id).await?;
    let model = bridge_common::models::get_for_chat(&pool, crate::CID, &sett, &chat).await?;

    // TODO: refactor error
    let api_key = sett
        .api_keys
        .get(&model.provider)
        .with_context(|| format!("Failed to get api key for provider: {:?}", model.provider))?;

    match chat.kind {
        bridge_common::types::chats::Kind::Direct => {
            bridge_common::chats::create_completion(
                &pool,
                &channel,
                crate::CID,
                crate::UID,
                chat.id,
                CreateCompletionParams::default(),
                &model,
                api_key,
                &crate::USER_AGENT,
            )
            .await
            .context("Failed to get chat completion")?;

            generate_chat_title(request.chat_id, channel, pool, settings).await?;
        }
        bridge_common::types::chats::Kind::Execution => {
            let task = repo::tasks::get_by_execution_chat_id(&*pool, crate::CID, chat.id)
                .await
                .context("Failed to `get_by_execution_chat_id`")?;

            if task.status != bridge_common::types::tasks::Status::InProgress
                || task.status != bridge_common::types::tasks::Status::ToDo
            {
                let task = repo::tasks::execute(&*pool, crate::CID, task.id).await?;
                channel.emit(crate::UID, Event::TaskUpdated(&task)).await?;
            }
        }
        bridge_common::types::chats::Kind::Control => {
            error!("Control chats are not yet supported");
        }
    }

    Ok(())
}

/// Generates a title for a chat.
///
/// The function will ask LLM to give chat a title if all the following conditions are met:
///
/// * The chat has one message from user
/// * The chat has one message from assistant
/// * Last message in the chat is from assistant
/// * The chat has no title yet
///
/// # Errors
///
/// Returns error if there was a problem while generating chat title.
#[instrument(skip_all)]
async fn generate_chat_title(
    chat_id: i32,
    channel: State<'_, Channel>,
    pool: State<'_, DbPool>,
    settings: State<'_, RwLock<Settings>>,
) -> Result<()> {
    let mut chat = repo::chats::get(&*pool, crate::CID, chat_id).await?;
    trace!("Chat: {:?}", chat);

    if !chat.title.is_empty() {
        debug!("Chat already has a title");
        return Ok(());
    }

    debug!("Generating chat title");

    let settings_guard = settings.read().await;

    let messages = repo::messages::list(&*pool, crate::CID, ListParams { chat_id }).await?;

    let model =
        bridge_common::models::get_for_chat(&pool, crate::CID, &settings_guard, &chat).await?;

    let api_key = settings_guard
        .api_keys
        .get(&model.provider)
        .with_context(|| format!("Failed to get api key for provider: {:?}", model.provider))?;

    let title = match bridge_common::messages::generate_chat_title(
        messages,
        &model,
        api_key,
        &crate::USER_AGENT,
    )
    .await
    {
        Ok(title) => title,
        Err(e) => {
            return match e {
                bridge_common::errors::Error::Messages(e) => match e {
                    bridge_common::messages::Error::TooFewMessages(_)
                    | bridge_common::messages::Error::NoSuitableMessages
                    | bridge_common::messages::Error::LastMessageNotFromAssistant => {
                        warn!("Failed: {}", e);

                        Ok(())
                    }
                    _ => Err(bridge_common::errors::Error::Messages(e).into()),
                },
                _ => Err(e.into()),
            }
        }
    };

    repo::chats::update_title(&*pool, crate::CID, chat_id, &title).await?;
    chat = repo::chats::get(&*pool, crate::CID, chat_id).await?;

    channel.emit(crate::UID, Event::ChatUpdated(&chat)).await?;

    Ok(())
}

/// Approves tool call, actually runs it and sends result to LLM.
///
/// # Errors
///
/// Returns error if there was a problem while performing tool call.
// TODO(ri-nat): refactor this function.
#[allow(clippy::too_many_lines)]
#[instrument(skip_all)]
#[tauri::command]
pub async fn approve_tool_call(
    message_id: i64,
    pool: State<'_, DbPool>,
    settings: State<'_, RwLock<Settings>>,
    channel: State<'_, Channel>,
    app_handle: AppHandle,
) -> Result<()> {
    debug!("Approving tool call");

    let mut message = repo::messages::get(&*pool, crate::CID, message_id).await?;

    // Check if message is waiting for tool call
    if message.status != Status::WaitingForToolCall {
        return Err(anyhow!("Message is not waiting for tool call").into());
    }

    // Check if message is a last message in chat
    let last_message_id = repo::messages::get_last_message_id(&*pool, crate::CID, message.chat_id)
        .await?
        .context("Failed to get last message id")?;

    // If it's not, mark message as completed and return error
    if message.id != last_message_id {
        // Mark message as completed
        repo::messages::update_status(&*pool, crate::CID, message.id, Status::Completed).await?;

        // Emit event.
        message.status = Status::Completed;
        channel
            .emit(crate::UID, Event::MessageUpdated(&message))
            .await?;

        return Err(anyhow!("Message is not a last message in chat").into());
    }

    let app_local_data_dir = app_handle
        .path_resolver()
        .app_local_data_dir()
        .expect("Failed to get app local data dir");

    // Execute abilities
    bridge_common::abilities::execute_for_message(
        &pool,
        &channel,
        crate::CID,
        crate::UID,
        &app_local_data_dir,
        &message,
    )
    .await?;

    // Emit event
    message.status = Status::Completed;
    channel
        .emit(crate::UID, Event::MessageUpdated(&message))
        .await?;

    let sett = settings.read().await.clone();

    let model = bridge_common::models::get_default(&pool, crate::CID, &sett).await?;
    let api_key = sett
        .api_keys
        .get(&model.provider)
        .with_context(|| format!("Failed to get api key for provider: {:?}", model.provider))?;

    bridge_common::chats::create_completion(
        &pool,
        &channel,
        crate::CID,
        crate::UID,
        message.chat_id,
        CreateCompletionParams::default(),
        &model,
        api_key,
        &crate::USER_AGENT,
    )
    .await
    .context("Failed to get chat completion")?;

    generate_chat_title(message.chat_id, channel, pool, settings).await?;

    Ok(())
}

/// Deny tool call
///
/// # Errors
///
/// Returns error if message with given id does not exist.
#[instrument(skip_all)]
#[tauri::command]
pub async fn deny_tool_call(
    message_id: i64,
    pool: State<'_, DbPool>,
    settings: State<'_, RwLock<Settings>>,
    channel: State<'_, Channel>,
) -> Result<()> {
    debug!("Denying tool call");

    let mut tx = pool.begin().await.context("Failed to begin transaction")?;
    let mut message = repo::messages::get(&mut *tx, crate::CID, message_id).await?;

    // Ensure the message is waiting for a tool call
    if message.status != Status::WaitingForToolCall {
        return Err(anyhow!("Message is not waiting for tool call").into());
    }

    // Update the message status to ToolCallDenied
    repo::messages::update_status(&mut *tx, crate::CID, message.id, Status::ToolCallDenied).await?;

    // Create a new message indicating the tool call was denied
    let denied_messages =
        repo::messages::create_tool_call_denied(&mut *tx, crate::CID, &message).await?;

    // Commit the transaction
    tx.commit().await.context("Failed to commit transaction")?;

    message.status = Status::ToolCallDenied;

    channel
        .emit(crate::UID, Event::MessageUpdated(&message))
        .await?;

    for denied_message in denied_messages {
        channel
            .emit(crate::UID, Event::MessageCreated(&denied_message))
            .await?;
    }

    let chat = repo::chats::get(&*pool, crate::CID, message.chat_id).await?;

    let sett = settings.read().await.clone();
    let model = bridge_common::models::get_for_chat(&pool, crate::CID, &sett, &chat)
        .await
        .context("Failed to get model for chat")?;
    let api_key = sett
        .api_keys
        .get(&model.provider)
        .with_context(|| format!("Failed to get api key for provider: {:?}", model.provider))?;

    bridge_common::chats::create_completion(
        &pool,
        &channel,
        crate::CID,
        crate::UID,
        message.chat_id,
        CreateCompletionParams::default(),
        &model,
        api_key,
        &crate::USER_AGENT,
    )
    .await
    .context("Failed to get chat completion")?;

    generate_chat_title(message.chat_id, channel, pool, settings).await?;

    Ok(())
}

/// Delete message by id.
///
/// # Errors
///
/// Returns error if there was a problem while deleting message.
#[instrument(skip_all)]
#[tauri::command]
pub async fn delete_message(id: i64, pool: State<'_, DbPool>) -> Result<()> {
    debug!("Deleting message");

    repo::messages::delete(&*pool, crate::CID, id).await?;

    Ok(())
}

/// Update message content by id.
///
/// # Errors
///
/// Returns error if there was a problem while updating message content.
#[instrument(skip_all)]
#[tauri::command]
pub async fn update_message_content(
    id: i64,
    content: String,
    pool: State<'_, DbPool>,
) -> Result<Message> {
    debug!("Updating message content");

    let mut tx = pool.begin().await.context("Failed to begin transaction")?;
    let message = repo::messages::get(&mut *tx, crate::CID, id).await?;

    if message.role != Role::System && message.role != Role::User {
        return Err(anyhow!(
            "Attempted to edit a message with an unsupported role: {:?}",
            message.role
        )
        .into());
    }

    let updated_message =
        repo::messages::update_message_content(&mut *tx, crate::CID, id, &content).await?;

    tx.commit().await.context("Failed to commit transaction")?;

    Ok(updated_message)
}

/// Get raw message content by id.
///
/// # Errors
///
/// Returns error if message with given id does not exist.
#[tauri::command]
pub async fn get_raw_message_content(id: i64, pool: State<'_, DbPool>) -> Result<String> {
    let message = repo::messages::get(&*pool, crate::CID, id)
        .await
        .with_context(|| "Failed to get message")?;

    Ok(message.content.unwrap_or_default())
}
