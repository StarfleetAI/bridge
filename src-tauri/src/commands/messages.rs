// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Manager, State, Window};
use tokio::{spawn, sync::RwLock};
use tracing::instrument;
use tracing::{debug, trace};

use crate::abilities::execute;
use crate::repo::models;
use crate::{
    clients::openai::{
        Client, CreateChatCompletionRequest, FunctionCall, Tool, ToolCall, ToolType,
    },
    errors, messages,
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
#[instrument(skip(pool, settings, window))]
pub async fn create_message(
    window: Window,
    request: CreateMessage,
    pool: State<'_, DbPool>,
    settings: State<'_, RwLock<Settings>>,
) -> Result<()> {
    debug!("Creating message");

    let mut tx = pool.begin().await.context("Failed to begin transaction")?;

    // Retrieve the last message for the chat
    let last_message_id = repo::messages::get_last_message_id(&mut *tx, request.chat_id).await?;
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

    get_chat_completion(
        request.chat_id,
        window.clone(),
        pool.clone(),
        settings.clone(),
    )
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

    trace!("Messages so far: {:?}", messages);

    let mut req_messages = messages
        .into_iter()
        .map(crate::clients::openai::Message::try_from)
        .collect::<std::result::Result<Vec<_>, _>>()?;

    req_messages.push(crate::clients::openai::Message::User {
        content: "Provide a short title for the current conversation (4-6 words)".to_string(),
        name: None,
    });

    let model_full_name = if chat.model_full_name.is_none() {
        settings_guard.default_model()
    } else {
        chat.model_full_name.as_ref().unwrap()
    };

    let model = models::get(&*pool, model_full_name)
        .await
        .context("Failed to get model")?;

    // Send request to LLM
    let client = Client::new(
        settings_guard
            .openai_api_key
            .as_ref()
            .context("Failed to get openai api key")?,
    );

    let response = client
        .create_chat_completion(CreateChatCompletionRequest {
            model: model.name,
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
    let last_message_id = repo::messages::get_last_message_id(&*pool, message.chat_id).await?;

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

    // Load agent abilities
    let abilities = match message.agent_id {
        Some(agent_id) => repo::abilities::list_for_agent(&*pool, agent_id).await?,
        None => return Err(anyhow!("Agent is not set for the message").into()),
    };

    let Some(tool_calls) = &message.tool_calls else {
        return Err(anyhow!("Tool calls are not set for the message").into());
    };

    let tool_calls: Vec<ToolCall> =
        serde_json::from_str(tool_calls).context("Failed to parse tool calls")?;

    let python_path_string = settings
        .read()
        .await
        .python_path
        .as_ref()
        .context("Failed to get python path")?
        .to_string();
    let app_local_data_dir = app_handle
        .path_resolver()
        .app_local_data_dir()
        .context("Failed to get app local data dir")?;

    let mut handles = Vec::with_capacity(tool_calls.len());
    for tool_call in tool_calls {
        let abilities = abilities.clone();
        let app_local_data_dir = app_local_data_dir.clone();
        let msg = message.clone();
        let tc = tool_call.clone();
        let python_path_str = python_path_string.clone();

        let handle = spawn(async move {
            let output = execute(abilities, app_local_data_dir, msg, tc, python_path_str).await?;
            // Wrap output in a code block
            //
            // TODO: This is a temporary solution. It's better to wrap it on before markdown-2-html
            //       processing, but it requires writing custom Serializer for Message.
            let output = format!("```\n{output}\n```");
            Ok::<_, anyhow::Error>(CreateParams {
                chat_id: message.chat_id,
                status: Status::Completed,
                role: Role::Tool,
                content: Some(output),
                tool_call_id: Some(tool_call.id),

                ..Default::default()
            })
        });

        handles.push(handle);
    }

    for handle in handles {
        let params = handle.await??;
        let results_message = repo::messages::create(&*pool, params).await?;

        // Emit event
        window
            .emit_all("messages:created", &results_message)
            .context("Failed to emit event")?;
    }

    // Mark message as completed
    repo::messages::update_status(&*pool, message.id, Status::Completed).await?;

    // Emit event
    message.status = Status::Completed;
    window
        .emit_all("messages:updated", &message)
        .context("Failed to emit event")?;

    get_chat_completion(
        message.chat_id,
        window.clone(),
        pool.clone(),
        settings.clone(),
    )
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

    get_chat_completion(
        message.chat_id,
        window.clone(),
        pool.clone(),
        settings.clone(),
    )
    .await
    .context("Failed to get chat completion for chat")?;

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
    let mut tx = pool.begin().await.context("Failed to begin transaction")?;

    repo::messages::delete(&mut *tx, id).await?;

    tx.commit().await.context("Failed to commit transaction")?;

    Ok(())
}

/// Edit message by id.
///
/// # Errors
///
/// Returns error if there was a problem while editing message.
#[instrument(skip(pool))]
#[tauri::command]
pub async fn update_message_content(
    id: i64,
    content: String,
    pool: State<'_, DbPool>,
) -> Result<Message> {
    let mut tx = pool.begin().await.context("Failed to begin transaction")?;
    // check if message role is system or user, if not return error
    let message = repo::messages::get(&mut *tx, id).await?;

    if message.role != Role::System && message.role != Role::User {
        return Err(anyhow!("Message role is not system or user").into());
    }

    let updated_message = repo::messages::update_message_content(&mut *tx, id, &content).await?;

    tx.commit().await.context("Failed to commit transaction")?;

    Ok(updated_message)
}

/// Does the whole chat completion routine.
// TODO: refactor this function.
#[instrument(skip(window, pool, settings))]
#[allow(clippy::too_many_lines)]
async fn get_chat_completion(
    chat_id: i64,
    window: Window,
    pool: State<'_, DbPool>,
    settings: State<'_, RwLock<Settings>>,
) -> Result<()> {
    debug!("Getting chat completion");
    let settings_guard = settings.read().await;

    let mut tx = pool.begin().await.context("Failed to begin transaction")?;

    let messages = repo::messages::list(&mut *tx, ListParams { chat_id }).await?;
    trace!("Messages so far: {:?}", messages);

    // Get current agent.
    let agent = repo::agents::get_for_chat(&mut *tx, chat_id).await?;
    let abilities = repo::abilities::list_for_agent(&mut *tx, agent.id).await?;

    let req_messages = messages
        .into_iter()
        .map(crate::clients::openai::Message::try_from)
        .collect::<std::result::Result<Vec<_>, _>>()?;

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
    .context("Failed to insert dummy assistant message")?;

    tx.commit().await.context("Failed to commit transaction")?;

    window
        .emit_all("messages:created", &message)
        .context("Failed to emit event")?;

    // Send request to LLM
    let client = Client::new(
        match settings_guard
            .openai_api_key
            .as_ref()
            .context("Failed to get openai api key")
        {
            Ok(api_key) => api_key,
            Err(err) => {
                fail_message(&window, &pool, &mut message).await?;

                return Err(err.into());
            }
        },
    );

    let mut tools = None;
    if !abilities.is_empty() {
        tools = Some(
            match abilities
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
                .collect::<Result<Vec<Tool>>>()
            {
                Ok(tools) => tools,
                Err(err) => {
                    fail_message(&window, &pool, &mut message).await?;

                    return Err(err);
                }
            },
        );

        debug!("Tools: {:?}", tools);
    }

    let model = models::get(&*pool, settings_guard.default_model())
        .await
        .context("Failed to get model")?;

    let mut response = match client
        .create_chat_completion_stream(CreateChatCompletionRequest {
            model: model.name,
            messages: req_messages,
            stream: true,
            tools,
        })
        .await
        .context("Failed to create chat completion")
    {
        Ok(response) => response,
        Err(err) => {
            fail_message(&window, &pool, &mut message).await?;

            return Err(err.into());
        }
    };

    let mut chunk_remainder = String::new();

    while let Some(chunk) = match response.chunk().await.context("Failed to get chunk") {
        Ok(chunk) => chunk,
        Err(err) => {
            fail_message(&window, &pool, &mut message).await?;

            return Err(err.into());
        }
    } {
        // TODO: come up with a more efficient way to split chunks.
        chunk_remainder.push_str(&String::from_utf8_lossy(&chunk));
        let chunk = chunk_remainder.clone();
        chunk_remainder = String::new();
        debug!("RAW chunk: {:?}", chunk);

        let chunks = chunk
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

                if let Err(err) = repo::messages::update_with_completion_result(
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
                .context("Failed to update assistant message")
                {
                    fail_message(&window, &pool, &mut message).await?;

                    return Err(err.into());
                };
            } else {
                match apply_completion_chunk(&mut message, chunk) {
                    Err(errors::Error::Messages(messages::Error::ChunkDeserialization(_))) => {
                        debug!("Error parsing chunk, might be incomplete, pushing to remainder");
                        chunk_remainder = chunk.to_string();
                    }
                    Err(err) => {
                        fail_message(&window, &pool, &mut message).await?;

                        return Err(err);
                    }
                    _ => {}
                };
            }

            if let Err(err) = window
                .emit_all("messages:updated", &message)
                .context("Failed to emit event")
            {
                fail_message(&window, &pool, &mut message).await?;

                return Err(err.into());
            };
        }
    }

    Ok(())
}

async fn fail_message(window: &Window, pool: &DbPool, message: &mut Message) -> Result<()> {
    repo::messages::update_status(pool, message.id, Status::Failed).await?;
    message.status = Status::Failed;

    window
        .emit_all("messages:updated", &message)
        .context("Failed to emit event")?;

    Ok(())
}

#[allow(clippy::too_many_lines)]
#[instrument(skip(message))]
fn apply_completion_chunk(message: &mut Message, chunk: &str) -> Result<()> {
    debug!("Applying completion chunk");
    let mut current_tool_call = None;

    if let Some(tool_calls_str) = &message.tool_calls {
        let tool_calls: Vec<ToolCall> =
            serde_json::from_str(tool_calls_str).context("Failed to parse tool calls")?;

        current_tool_call = tool_calls.into_iter().last();
        trace!("Last tool call: {:?}", current_tool_call);
    }

    let completion: Value = serde_json::from_str(
        chunk
            .trim()
            .strip_prefix("data: ")
            .context(format!("Failed to strip prefix for chunk: {chunk}"))?,
    )
    .map_err(messages::Error::ChunkDeserialization)?;

    if let Some(choices) = completion.get("choices") {
        trace!("Choices: {:?}", choices);

        if let Some(delta) = choices[0].get("delta") {
            trace!("Delta: {:?}", delta);

            match delta.get("content") {
                Some(content) if content.is_string() => {
                    trace!("Content: {:?}", content);

                    message.content = Some(match &message.content {
                        Some(existed) => {
                            existed.to_owned()
                                + content.as_str().context("Failed to get content as str")?
                        }
                        None => content
                            .as_str()
                            .context("Failed to get content as str")?
                            .to_string(),
                    });
                }
                _ => {}
            }

            match delta.get("tool_calls") {
                Some(tool_calls) if tool_calls.is_array() => {
                    trace!("Tool call: {:?}", tool_calls[0]);

                    if current_tool_call.is_none() || tool_calls[0].get("id").is_some() {
                        trace!("Current tool call: {:?}", current_tool_call);
                        trace!("Get id: {:?}", tool_calls[0].get("id"));
                        current_tool_call = Some(ToolCall {
                            id: String::new(),
                            type_: ToolType::Function,
                            function: FunctionCall {
                                name: String::new(),
                                arguments: String::new(),
                            },
                        });
                    }

                    if let Some(id) = tool_calls[0].get("id") {
                        trace!("ID: {:?}", id);

                        current_tool_call
                            .as_mut()
                            .context("Failed to get tool call")?
                            .id
                            .push_str(id.as_str().context("Failed to get id as str")?);
                    }

                    if let Some(function) = tool_calls[0].get("function") {
                        trace!("Function: {:?}", function);

                        if let Some(name) = function.get("name") {
                            trace!("Name: {:?}", name);

                            current_tool_call
                                .as_mut()
                                .context("Failed to get tool call")?
                                .function
                                .name
                                .push_str(name.as_str().context("Failed to get name as str")?);
                        }

                        if let Some(arguments) = function.get("arguments") {
                            trace!("Arguments: {:?}", arguments);

                            current_tool_call
                                .as_mut()
                                .context("Failed to get tool call")?
                                .function
                                .arguments
                                .push_str(
                                    arguments
                                        .as_str()
                                        .context("Failed to get arguments as str")?,
                                );
                        }
                    }
                }
                _ => {}
            }
        }
    }

    if let Some(tool_call) = current_tool_call {
        let tool_calls = match &message.tool_calls {
            Some(tool_calls_str) => {
                let mut tc: Vec<ToolCall> =
                    serde_json::from_str(tool_calls_str).context("Failed to parse tool calls")?;

                if tool_call.id == tc.last().context("Last tool call is somehow None")?.id {
                    tc.pop();
                }

                tc.push(tool_call);
                tc
            }
            None => vec![tool_call],
        };

        trace!("Resulting tool calls: {:?}", tool_calls);

        message.tool_calls =
            Some(serde_json::to_string(&tool_calls).context("Failed to serialize tool calls")?);
    }

    Ok(())
}
