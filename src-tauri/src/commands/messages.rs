// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// TODO(ri-nat): I don't really know, why Clippy is mad about these here, but let make him quiet for now.
#![allow(clippy::used_underscore_binding)]

use std::path::PathBuf;

use anyhow::Context;
use askama::Template;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Manager, State, Window};
use tokio::{fs::create_dir_all, process::Command, sync::RwLock};

use crate::{
    clients::openai::{
        Client, CreateChatCompletionRequest, FunctionCall, Tool, ToolCall, ToolType,
    },
    errors,
    repo::{
        self,
        messages::{
            CreateParams, ListParams, Message, Role, Status, UpdateWithCompletionResultParams,
        },
    },
    settings::Settings,
    types::{DbPool, Result},
};

const CHUNK_SEPARATOR: &str = "\n\n";
const DONE_CHUNK: &str = "data: [DONE]";

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

#[derive(Template)]
#[template(path = "python/call_tools.py", escape = "none")]
struct CallToolsTemplate<'a> {
    code: &'a str,
    python_path: &'a str,
    tool_calls: &'a str,
}

/// List all messages.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_messages(request: ListMessages, pool: State<'_, DbPool>) -> Result<MessagesList> {
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
pub async fn create_message(
    window: Window,
    request: CreateMessage,
    pool: State<'_, DbPool>,
    settings: State<'_, RwLock<Settings>>,
) -> Result<()> {
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

    get_chat_completion(request.chat_id, window, pool, settings)
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
    pool: State<'_, DbPool>,
    settings: State<'_, RwLock<Settings>>,
    window: Window,
    app_handle: AppHandle,
) -> Result<()> {
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

    let settings_guard = settings.read().await;

    let Some(tool_calls) = &message.tool_calls else {
        return Err(anyhow::anyhow!("Tool calls are not set for the message").into());
    };

    let script_name = format!("script-{}.py", message.id);
    let call_tools_template = CallToolsTemplate {
        code: &code,
        tool_calls,
        python_path: settings_guard
            .python_path
            .as_ref()
            .with_context(|| "Failed to get python path")?,
    };
    let content = call_tools_template
        .render()
        .with_context(|| "Failed to render `call_tools` script")?;

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
    let output = match &settings_guard.python_path {
        Some(path) => Command::new(path)
            .current_dir(&workdir)
            .arg(&script_path)
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

    // Delete script
    tokio::fs::remove_file(&script_path)
        .await
        .with_context(|| "Failed to remove script from workdir")?;

    // Mark message as completed
    repo::messages::update_status(&mut *tx, message.id, Status::Completed).await?;

    // Emit event
    message.status = Status::Completed;
    window
        .emit_all("messages:updated", &message)
        .with_context(|| "Failed to emit event")?;

    let tool_call_id = results
        .as_object()
        .with_context(|| "Failed to get results object")?
        .keys()
        .next()
        .with_context(|| "Failed to get first key from results object")?
        .to_string();

    let content = results
        .get(&tool_call_id)
        .with_context(|| format!("Failed to get value for key {tool_call_id} from results object"))?
        .as_str()
        .with_context(|| {
            format!("Failed to get string value for key {tool_call_id} from results object")
        })?;

    // Save script output to a new message
    let results_message = repo::messages::create(
        &mut *tx,
        CreateParams {
            chat_id: message.chat_id,
            status: Status::Completed,
            role: Role::Tool,
            content: Some(content.to_string()),
            tool_call_id: Some(tool_call_id),

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

    drop(settings_guard);

    get_chat_completion(message.chat_id, window, pool, settings)
        .await
        .with_context(|| {
            format!(
                "Failed to get chat completion for chat with id {}",
                message.chat_id
            )
        })?;

    Ok(())
}


#[tauri::command]
pub async fn deny_tool_call(
    message_id: i64,
    pool: State<'_, DbPool>,
    window: Window,
) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let message = repo::messages::get(&mut *tx, message_id).await?;

    // Ensure the message is waiting for a tool call
    if message.status != Status::WaitingForToolCall {
        return Err(anyhow::anyhow!("Message is not waiting for tool call").into());
    }

    // Update the message status to ToolCallDenied
    repo::messages::update_status(&mut *tx, message.id, Status::ToolCallDenied).await?;

    // Assuming tool_call_id is already set in the original message
    let tool_call_id_clone = message.tool_call_id.clone().unwrap_or_default();

    // Create a new message indicating the tool call was denied
    let denied_message = repo::messages::create(
        &mut *tx,
        CreateParams {
            chat_id: message.chat_id,
            status: Status::ToolCallDenied,
            role: Role::Tool,
            content: Some("Tool call denied".to_string()),
            tool_call_id: Some(tool_call_id_clone), 

            ..Default::default()
        },
    )
    .await?;

    // Commit the transaction
    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    window
        .emit_all("messages:updated", &message)
        .with_context(|| "Failed to emit message update event")?;
    window
        .emit_all("messages:created", &denied_message)
        .with_context(|| "Failed to emit message creation event")?;

    Ok(())
}



/// Delete message by id.
///
/// # Errors
///
/// Returns error if there was a problem while deleting message.
#[tauri::command]
pub async fn delete_message(id: i64, pool: State<'_, DbPool>) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    repo::messages::delete(&mut *tx, id).await?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}

/// Does the whole chat completion routine.
// TODO: refactor this function.
#[allow(clippy::too_many_lines)]
async fn get_chat_completion(
    chat_id: i64,
    window: Window,
    pool: State<'_, DbPool>,
    settings: State<'_, RwLock<Settings>>,
) -> Result<()> {
    let settings_guard = settings.read().await;

    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let messages = repo::messages::list(&mut *tx, ListParams { chat_id }).await?;
    debug!("Messages so far: {:?}", messages);

    // Get current agent.
    let agent = repo::agents::get_for_chat(&mut *tx, chat_id).await?;
    let abilities = repo::abilities::list_for_agent(&mut *tx, agent.id).await?;

    let req_messages = messages
        .into_iter()
        .map(|message| crate::clients::openai::Message::try_from(message).unwrap())
        .collect();

    // Insert dummy message to chat.
    let mut message = repo::messages::create(
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

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

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

    let mut tools = None;
    if !abilities.is_empty() {
        tools = Some(
            abilities
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
                .collect::<Result<Vec<Tool>>>()?,
        );

        debug!("Tools: {:?}", tools);
    }

    let mut response = client
        .create_chat_completion_stream(CreateChatCompletionRequest {
            model: "gpt-4".to_string(),
            messages: req_messages,
            stream: true,
            tools,
        })
        .await
        .with_context(|| "Failed to create chat completion")?;

    while let Some(chunk) = response
        .chunk()
        .await
        .with_context(|| "Failed to get chunk")?
    {
        // TODO: come up with a more efficient way to split chunks.
        let chunk_str = String::from_utf8_lossy(&chunk);
        let chunks = chunk_str
            .split(CHUNK_SEPARATOR)
            .map(str::trim)
            .filter(|chunk| !chunk.is_empty())
            .collect::<Vec<&str>>();

        for chunk in chunks {
            if chunk == DONE_CHUNK {
                message.status = match message.tool_calls {
                    Some(_) => Status::WaitingForToolCall,
                    None => Status::Completed,
                };

                repo::messages::update_with_completion_result(
                    &*pool,
                    UpdateWithCompletionResultParams {
                        id: message.id,
                        status: message.status,
                        content: message.content.clone(),
                        prompt_tokens: None,
                        completion_tokens: None,
                        tool_calls: message.tool_calls.clone(),
                    },
                )
                .await
                .with_context(|| "Failed to update assistant message")?;
            } else {
                apply_completion_chunk(&mut message, chunk)
                    .with_context(|| "Failed to update message with completion chunk")?;
            }

            window
                .emit_all("messages:updated", &message)
                .with_context(|| "Failed to emit event")?;
        }
    }

    Ok(())
}

fn apply_completion_chunk(message: &mut Message, chunk: &str) -> Result<()> {
    let mut message_tool_call = None;

    if let Some(tool_calls_str) = &message.tool_calls {
        let tool_calls: Vec<ToolCall> =
            serde_json::from_str(tool_calls_str).with_context(|| "Failed to parse tool calls")?;

        message_tool_call = tool_calls.into_iter().next();
    }

    let completion: Value = serde_json::from_str(
        chunk
            .trim()
            .strip_prefix("data: ")
            .with_context(|| format!("Failed to strip prefix for chunk: {chunk}"))?,
    )
    .with_context(|| format!("Failed to parse OpenAI API response chunk: {chunk}"))?;

    if let Some(choices) = completion.get("choices") {
        debug!("Choices: {:?}", choices);

        if let Some(delta) = choices[0].get("delta") {
            debug!("Delta: {:?}", delta);

            match delta.get("content") {
                Some(content) if content.is_string() => {
                    debug!("Content: {:?}", content);

                    message.content = Some(match &message.content {
                        Some(existed) => {
                            existed.to_owned()
                                + content
                                    .as_str()
                                    .with_context(|| "Failed to get content as str")?
                        }
                        None => content
                            .as_str()
                            .with_context(|| "Failed to get content as str")?
                            .to_string(),
                    });
                }
                _ => {}
            }

            match delta.get("tool_calls") {
                Some(tool_calls) if tool_calls.is_array() => {
                    debug!("Tool call: {:?}", tool_calls[0]);

                    if message_tool_call.is_none() {
                        message_tool_call = Some(ToolCall {
                            id: String::new(),
                            type_: ToolType::Function,
                            function: FunctionCall {
                                name: String::new(),
                                arguments: String::new(),
                            },
                        });
                    }

                    if let Some(function) = tool_calls[0].get("function") {
                        debug!("Function: {:?}", function);

                        if let Some(id) = function.get("id") {
                            debug!("ID: {:?}", id);

                            message_tool_call
                                .as_mut()
                                .with_context(|| "Failed to get tool call")?
                                .id
                                .push_str(id.as_str().with_context(|| "Failed to get id as str")?);
                        }

                        if let Some(name) = function.get("name") {
                            debug!("Name: {:?}", name);

                            message_tool_call
                                .as_mut()
                                .with_context(|| "Failed to get tool call")?
                                .function
                                .name
                                .push_str(
                                    name.as_str().with_context(|| "Failed to get name as str")?,
                                );
                        }

                        if let Some(arguments) = function.get("arguments") {
                            debug!("Arguments: {:?}", arguments);

                            message_tool_call
                                .as_mut()
                                .with_context(|| "Failed to get tool call")?
                                .function
                                .arguments
                                .push_str(
                                    arguments
                                        .as_str()
                                        .with_context(|| "Failed to get arguments as str")?,
                                );
                        }
                    }
                }
                _ => {}
            }
        }
    }

    if let Some(tool_call) = &message_tool_call {
        message.tool_calls = Some(
            serde_json::to_string(&vec![tool_call])
                .with_context(|| "Failed to serialize tool calls")?,
        );
    }

    Ok(())
}
