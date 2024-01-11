// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// TODO(ri-nat): I don't really know, why Clippy is mad about these here, but let make him quiet for now.
#![allow(clippy::used_underscore_binding)]

use std::path::PathBuf;

use anyhow::Context;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{App, Manager, State, Window};
use tokio::{fs::create_dir_all, process::Command, sync::RwLock};

use crate::{
    clients::openai::{Client, CreateChatCompletionRequest, Tool},
    errors,
    repo::{
        self,
        messages::{
            CreateParams, ListParams, Message, Role, Status, UpdateWithCompletionResultParams,
        },
    },
    settings::Settings,
    types::{DbMutex, Result},
};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteMessage {
    pub id: i64,
}

/// List all messages.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_messages(
    request: ListMessages,
    pool_mutex: State<'_, DbMutex>,
) -> Result<MessagesList> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let messages = repo::messages::list(
        pool,
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
///
/// # Panics
///
/// Panics if there is an error when converting message from a database row to a API-compatible
/// message. Should never happen.
// TODO: refactor this function.
#[tauri::command]
pub async fn create_message(
    window: Window,
    request: CreateMessage,
    pool_mutex: State<'_, DbMutex>,
    settings: State<'_, RwLock<Settings>>,
) -> Result<()> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

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

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    window
        .emit_all("messages:created", &message)
        .with_context(|| "Failed to emit event")?;

    drop(pool_guard);

    get_chat_completion(request.chat_id, window, pool_mutex, settings)
        .await
        .with_context(|| {
            format!(
                "Failed to get chat completion for chat with id {}",
                request.chat_id
            )
        })?;

    Ok(())
}

/// Approves tool call, actually runs it and sends result to LLM.
///
/// # Errors
///
/// Returns error if there was a problem while performing tool call.
// TODO(ri-nat): refactor this function.
#[allow(clippy::too_many_lines)]
#[tauri::command]
pub async fn approve_tool_call(
    message_id: i64,
    pool_mutex: State<'_, DbMutex>,
    settings: State<'_, RwLock<Settings>>,
    window: Window,
    app: App,
) -> Result<()> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let mut message = repo::messages::get(&mut *tx, message_id).await?;

    // Check if message is waiting for tool call
    if message.status != Status::WaitingForToolCall {
        return Err(anyhow::anyhow!("Message is not waiting for tool call").into());
    }

    // Check if message is a last message in chat
    let last_message_id = repo::messages::get_last_message_id(&mut *tx, message.chat_id).await?;

    if message.id != last_message_id {
        // Mark message as completed
        repo::messages::update_status(&mut *tx, message.id, Status::Completed).await?;

        // Emit event.
        message.status = Status::Completed;
        window
            .emit_all("messages:updated", &message)
            .with_context(|| "Failed to emit event")?;

        return Err(anyhow::anyhow!("Message is not a last message in chat").into());
    }

    // Load agent abilities
    let ablts = match message.agent_id {
        Some(agent_id) => repo::abilities::list_for_agent(&mut *tx, agent_id).await?,
        None => return Err(anyhow::anyhow!("Agent is not set for the message").into()),
    };

    // Join the abilities code into one string
    let code = ablts
        .iter()
        .map(|ability| ability.code.as_str())
        .collect::<Vec<&str>>()
        .join("\n\n");

    let app_handle = app.handle();
    let app_local_data_dir = app_handle
        .path_resolver()
        .app_local_data_dir()
        .with_context(|| "Failed to get app local data dir")?;
    let workdir_name = format!("workdir-{}", message.chat_id);

    // Build workdir path
    let mut workdir = PathBuf::new();
    workdir.push(app_local_data_dir);
    workdir.push(workdir_name);

    debug!("Workdir: {:?}", workdir);

    if !workdir.exists() {
        create_dir_all(&workdir)
            .await
            .with_context(|| "Failed to create workdir")?;
    }

    let Some(tool_calls) = &message.tool_calls else {
        return Err(anyhow::anyhow!("Tool calls are not set for the message").into());
    };

    let script_name = format!("script-{}.py", message.id);
    let content = format!(
        r#"
import json

{code}

tool_calls = {tool_calls}
results = {{}}
for tool_call in tool_calls:
    name = tool_call['function']['name']
    arguments = json.loads(tool_call['function']['arguments'])

    try:
        results[tool_call.id] = globals()[name](**arguments)
    except Exception as e:
        results[tool_call.id] = str(e)
        break

print(json.dumps(results))
"#
    );

    debug!("Script name: {}", script_name);
    debug!("Script content: {}", content);

    // Write script to workdir
    let mut script_path = workdir.clone();
    script_path.push(script_name);
    debug!("Script path: {:?}", script_path);

    tokio::fs::write(&script_path, content)
        .await
        .with_context(|| "Failed to write script to workdir")?;

    // Run script
    let settings_guard = settings.read().await;
    let output = match &settings_guard.python_path {
        Some(path) => Command::new(path)
            .current_dir(&workdir)
            .arg(script_path)
            .output()
            .await
            .with_context(|| "Failed to execute tool_calls script")?,
        None => return Err(anyhow::anyhow!("Python path is not set").into()),
    };

    debug!("Function call script output: {:?}", output);

    // Ensure that script was executed successfully
    let results: Value = serde_json::from_slice(&output.stdout)
        .with_context(|| "Failed to parse tool_calls script output")?;
    debug!("Parsed results: {:?}", results);

    // Mark message as completed
    repo::messages::update_status(&mut *tx, message.id, Status::Completed).await?;

    // Emit event
    message.status = Status::Completed;
    window
        .emit_all("messages:updated", &message)
        .with_context(|| "Failed to emit event")?;

    // Save script output to a new message
    let content = serde_json::to_string(&results)
        .with_context(|| "Failed to serialize tool_calls script results")?;
    let results_message = repo::messages::create(
        &mut *tx,
        CreateParams {
            chat_id: message.chat_id,
            status: Status::Completed,
            role: Role::Tool,
            content: Some(content),

            ..Default::default()
        },
    )
    .await?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    // Emit event
    window
        .emit_all("messages:created", &results_message)
        .with_context(|| "Failed to emit event")?;

    drop(pool_guard);
    drop(settings_guard);

    get_chat_completion(message.chat_id, window, pool_mutex, settings)
        .await
        .with_context(|| {
            format!(
                "Failed to get chat completion for chat with id {}",
                message.chat_id
            )
        })?;

    Ok(())
}

/// Delete message by id.
///
/// # Errors
///
/// Returns error if there was a problem while deleting message.
#[tauri::command]
pub async fn delete_message(request: DeleteMessage, pool_mutex: State<'_, DbMutex>) -> Result<()> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    repo::messages::delete(&mut *tx, request.id).await?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}

/// Does the whole chat completion routine.
async fn get_chat_completion(
    chat_id: i64,
    window: Window,
    pool_mutex: State<'_, DbMutex>,
    settings: State<'_, RwLock<Settings>>,
) -> Result<()> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;
    let settings_guard = settings.read().await;

    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let messages = repo::messages::list(&mut *tx, ListParams { chat_id }).await?;

    debug!("Messages so far: {:?}", messages);

    // Get current agent.
    let agent = repo::agents::get_for_chat(&mut *tx, chat_id).await?;

    let req_messages = messages
        .into_iter()
        .map(|message| crate::clients::openai::Message::try_from(message).unwrap())
        .collect();

    // Insert dummy message to chat.
    let message = repo::messages::create(
        &mut *tx,
        CreateParams {
            chat_id,
            agent_id: Some(agent.id),
            status: Status::Writing,
            role: Role::Assistant,
            ..Default::default()
        },
    )
    .await
    .with_context(|| "Failed to insert dummy assistant message")?;

    window
        .emit_all("messages:created", &message)
        .with_context(|| "Failed to emit event")?;

    // Send request to LLM
    let client = Client::new(
        settings_guard
            .openai_api_key
            .as_ref()
            .with_context(|| "Failed to get openai api key")?,
    );

    let abilities = repo::abilities::list_for_agent(&mut *tx, agent.id).await?;

    let tools: Vec<Tool> = abilities
        .into_iter()
        .map(
            |ability| match serde_json::from_str(&ability.parameters_json) {
                Ok(function) => Ok(Tool {
                    type_: "function".to_string(),
                    function,
                }),
                Err(err) => Err(errors::Error::Internal(err.into())),
            },
        )
        .collect::<Result<Vec<Tool>>>()?;

    debug!("Tools: {:?}", tools);

    let completion = client
        .create_chat_completion(CreateChatCompletionRequest {
            model: "gpt-4".to_string(),
            messages: req_messages,
            tools,
        })
        .await
        .with_context(|| "Failed to create chat completion")?;

    // Update message in chat.
    //
    // We're only using the first message for now.
    let message = match &completion.choices[0].message {
        crate::clients::openai::Message::Assistant {
            content,
            tool_calls,
            ..
        } => {
            let mut status = Status::Completed;
            let tool_calls = match &tool_calls {
                Some(calls) => {
                    status = Status::WaitingForToolCall;

                    Some(
                        serde_json::to_string(&calls)
                            .with_context(|| "Failed to serialize tool calls")?,
                    )
                }
                None => None,
            };

            repo::messages::update_with_completion_result(
                &mut *tx,
                UpdateWithCompletionResultParams {
                    id: message.id,
                    status,
                    content: content.clone(),
                    prompt_tokens: Some(i64::from(completion.usage.prompt_tokens)),
                    completion_tokens: Some(i64::from(completion.usage.completion_tokens)),
                    tool_calls,
                },
            )
            .await
            .with_context(|| "Failed to update assistant message")
        }
        _ => return Err(anyhow::anyhow!("Unexpected message type").into()),
    }?;

    window
        .emit_all("messages:updated", &message)
        .with_context(|| "Failed to emit event")?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}
