// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State, Window};
use tokio::sync::RwLock;
use tracing::{debug, trace};
use tracing::instrument;

use crate::{
    clients::openai::{Client, CreateChatCompletionRequest},
    repo::{
        self,
        messages::{CreateParams, ListParams, Message, Role, Status},
    },
    settings::Settings,
    types::{DbPool, Result},
};
use crate::abilities::{self};
use crate::chats;
use crate::chats::GetCompletionParams;
use crate::repo::models;

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
#[instrument(skip(app_handle, window, pool, settings))]
pub async fn create_message(
    app_handle: AppHandle,
    window: Window,
    request: CreateMessage,
    pool: State<'_, DbPool>,
    settings: State<'_, RwLock<Settings>>,
) -> Result<()> {
    debug!("Creating message");

    let mut tx = pool.begin().await.context("Failed to begin transaction")?;

    // Retrieve the last message for the chat
    let last_message_id = repo::messages::get_last_message_id(&mut *tx, request.chat_id)
        .await?
        .context("Failed to get last message id")?;
    let mut last_message = repo::messages::get(&mut *tx, last_message_id).await?;

    // If last message status is waiting for tool call, deny it
    if last_message.status == Status::WaitingForToolCall {
        // Update the message status to ToolCallDenied
        repo::messages::update_status(&mut *tx, last_message_id, Status::ToolCallDenied).await?;
        // Create a new message indicating the tool call was denied
        let denied_messages =
            repo::messages::create_tool_call_denied(&mut tx, &last_message).await?;

        last_message.status = Status::ToolCallDenied;
        window
            .emit_all("messages:updated", &last_message)
            .context("Failed to emit message update event")?;

        for denied_message in denied_messages {
            window
                .emit_all("messages:created", &denied_message)
                .context("Failed to emit message creation event")?;
        }
    }

    let message = repo::messages::create(
        &mut *tx,
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

    window
        .emit_all("messages:created", &message)
        .context("Failed to emit event")?;

    chats::get_completion(&app_handle, request.chat_id, GetCompletionParams::default())
        .await
        .context("Failed to get chat completion")?;

    generate_chat_title(request.chat_id, window, pool, settings).await?;

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
#[instrument(skip(window, pool, settings))]
async fn generate_chat_title(
    chat_id: i64,
    window: Window,
    pool: State<'_, DbPool>,
    settings: State<'_, RwLock<Settings>>,
) -> Result<()> {
    debug!("Generating chat title");

    let mut chat = repo::chats::get(&*pool, chat_id).await?;
    trace!("Chat: {:?}", chat);

    if !chat.title.is_empty() {
        debug!("Chat already has a title");
        return Ok(());
    }

    let settings_guard = settings.read().await;

    let messages = repo::messages::list(&*pool, ListParams { chat_id }).await?;

    if messages.len() < 3 {
        debug!("Chat has less than 3 messages");
        return Ok(());
    }

    let user_message = messages.iter().find(|message| message.role == Role::User);
    let assistant_message = messages
        .iter()
        .find(|message| message.role == Role::Assistant);

    if user_message.is_none() || assistant_message.is_none() {
        debug!("Chat has no user or assistant messages");
        return Ok(());
    }

    let last_message = messages.last().unwrap();

    if last_message.role != Role::Assistant {
        debug!("Last message in the chat is not from assistant");
        return Ok(());
    }

    // TODO: would be nice to skip `tool_output` messages here. This requires cleaning up
    //       the `tool_calls` in assistant messages.
    trace!("Messages so far: {:?}", messages);

    let mut req_messages = messages
        .into_iter()
        .map(crate::clients::openai::Message::try_from)
        .collect::<std::result::Result<Vec<_>, _>>()?;

    req_messages.push(crate::clients::openai::Message::User {
        content: "Provide a short title for the current conversation (4-6 words)".to_string(),
        name: None,
    });

    let model_full_name = match chat.model_full_name {
        Some(ref name) => name,
        None => settings_guard.default_model(),
    };

    let model = models::get(&*pool, model_full_name)
        .await
        .context("Failed to get model")?;

    let api_key = settings_guard
        .api_keys
        .get(&model.provider)
        .with_context(|| format!("Failed to get api key for provider: {:?}", model.provider))?;

    // Send request to LLM
    let client = Client::new(api_key, model.api_url_or_default());
    let response = client
        .create_chat_completion(CreateChatCompletionRequest {
            model: &model.name,
            messages: req_messages,
            stream: false,
            tools: None,
        })
        .await
        .context("Failed to create chat completion")?;

    let mut title = match &response.choices[0].message {
        crate::clients::openai::Message::Assistant { content, .. } => match content {
            Some(title) => title,
            _ => return Err(anyhow!("Received empty response from LLM").into()),
        },
        _ => return Err(anyhow!("Failed to get title from LLM").into()),
    }
    .to_string();

    // Clean up title
    if title.starts_with('"') && title.ends_with('"') {
        title = title
            .trim_start_matches('"')
            .trim_end_matches('"')
            .to_string();
    }

    repo::chats::update_title(&*pool, chat_id, &title).await?;
    chat = repo::chats::get(&*pool, chat_id).await?;

    window
        .emit_all("chats:updated", &chat)
        .context("Failed to emit message update event")?;

    Ok(())
}

/// Approves tool call, actually runs it and sends result to LLM.
///
/// # Errors
///
/// Returns error if there was a problem while performing tool call.
// TODO(ri-nat): refactor this function.
#[allow(clippy::too_many_lines)]
#[instrument(skip(window, pool, settings, app_handle))]
#[tauri::command]
pub async fn approve_tool_call(
    message_id: i64,
    pool: State<'_, DbPool>,
    settings: State<'_, RwLock<Settings>>,
    window: Window,
    app_handle: AppHandle,
) -> Result<()> {
    debug!("Approving tool call");

    let mut message = repo::messages::get(&*pool, message_id).await?;

    // Check if message is waiting for tool call
    if message.status != Status::WaitingForToolCall {
        return Err(anyhow!("Message is not waiting for tool call").into());
    }

    // Check if message is a last message in chat
    let last_message_id = repo::messages::get_last_message_id(&*pool, message.chat_id)
        .await?
        .context("Failed to get last message id")?;

    // If it's not, mark message as completed and return error
    if message.id != last_message_id {
        // Mark message as completed
        repo::messages::update_status(&*pool, message.id, Status::Completed).await?;

        // Emit event.
        message.status = Status::Completed;
        window
            .emit_all("messages:updated", &message)
            .context("Failed to emit event")?;

        return Err(anyhow!("Message is not a last message in chat").into());
    }

    // Execute abilities
    abilities::execute_for_message(&message, &app_handle).await?;

    // Emit event
    message.status = Status::Completed;
    window
        .emit_all("messages:updated", &message)
        .context("Failed to emit event")?;

    chats::get_completion(&app_handle, message.chat_id, GetCompletionParams::default())
        .await
        .context("Failed to get chat completion")?;

    generate_chat_title(message.chat_id, window, pool, settings).await?;

    Ok(())
}

/// Deny tool call
///
/// # Errors
///
/// Returns error if message with given id does not exist.
#[instrument(skip(window, pool, settings))]
#[tauri::command]
pub async fn deny_tool_call(
    message_id: i64,
    pool: State<'_, DbPool>,
    settings: State<'_, RwLock<Settings>>,
    app_handle: AppHandle,
    window: Window,
) -> Result<()> {
    debug!("Denying tool call");

    let mut tx = pool.begin().await.context("Failed to begin transaction")?;
    let mut message = repo::messages::get(&mut *tx, message_id).await?;

    // Ensure the message is waiting for a tool call
    if message.status != Status::WaitingForToolCall {
        return Err(anyhow!("Message is not waiting for tool call").into());
    }

    // Update the message status to ToolCallDenied
    repo::messages::update_status(&mut *tx, message.id, Status::ToolCallDenied).await?;

    // Create a new message indicating the tool call was denied
    let denied_message = repo::messages::create_tool_call_denied(&mut tx, &message).await?;

    // Commit the transaction
    tx.commit().await.context("Failed to commit transaction")?;

    message.status = Status::ToolCallDenied;

    window
        .emit_all("messages:updated", &message)
        .context("Failed to emit message update event")?;
    window
        .emit_all("messages:created", &denied_message)
        .context("Failed to emit message creation event")?;

    chats::get_completion(&app_handle, message.chat_id, GetCompletionParams::default())
        .await
        .context("Failed to get chat completion")?;

    generate_chat_title(message.chat_id, window, pool, settings).await?;

    Ok(())
}

/// Delete message by id.
///
/// # Errors
///
/// Returns error if there was a problem while deleting message.
#[instrument(skip(pool))]
#[tauri::command]
pub async fn delete_message(id: i64, pool: State<'_, DbPool>) -> Result<()> {
    debug!("Deleting message");

    repo::messages::delete(&*pool, id).await
}

/// Update message content by id.
///
/// # Errors
///
/// Returns error if there was a problem while updating message content.
#[instrument(skip(content, pool))]
#[tauri::command]
pub async fn update_message_content(
    id: i64,
    content: String,
    pool: State<'_, DbPool>,
) -> Result<Message> {
    debug!("Updating message content");

    let mut tx = pool.begin().await.context("Failed to begin transaction")?;
    let message = repo::messages::get(&mut *tx, id).await?;

    if message.role != Role::System && message.role != Role::User {
        return Err(anyhow!(
            "Attempted to edit a message with an unsupported role: {:?}",
            message.role
        )
        .into());
    }

    let updated_message = repo::messages::update_message_content(&mut *tx, id, &content).await?;

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
    let message = repo::messages::get(&*pool, id)
        .await
        .with_context(|| "Failed to get message")?;

    Ok(message.content.unwrap_or_default())
}
