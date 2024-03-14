// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use serde_json::Value;
use tauri::{AppHandle, Manager, State, Window};
use tokio::sync::RwLock;
use tracing::{debug, instrument, trace};

use crate::{
    clients::openai::{
        Client, CreateChatCompletionRequest, FunctionCall, Tool, ToolCall, ToolType,
    },
    errors, messages,
    repo::{
        self,
        abilities::Ability,
        messages::{
            CreateParams, ListParams, Message, Role, Status, UpdateWithCompletionResultParams,
        },
        models,
    },
    settings::Settings,
    types::{DbPool, Result},
};

const CHUNK_SEPARATOR: &str = "\n\n";
const DONE_CHUNK: &str = "data: [DONE]";

/// Does the whole chat completion routine.
// TODO: refactor this function.
#[instrument(skip(app_handle, messages_pre))]
#[allow(clippy::too_many_lines)]
pub async fn get_completion(
    chat_id: i64,
    app_handle: &AppHandle,
    messages_pre: Option<Vec<Message>>,
    abilities: Option<Vec<Ability>>,
) -> Result<()> {
    debug!("Getting chat completion");
    let pool: State<'_, DbPool> = app_handle.state();
    let settings: State<'_, RwLock<Settings>> = app_handle.state();

    let window = app_handle
        .get_window("main")
        .context("Failed to get main window")?;

    let settings_guard = settings.read().await;

    let mut tx = pool.begin().await.context("Failed to begin transaction")?;

    let mut messages = repo::messages::list(&mut *tx, ListParams { chat_id }).await?;
    if let Some(messages_pre) = messages_pre {
        messages = messages_pre.into_iter().chain(messages).collect();
    }

    trace!("Messages so far: {:?}", messages);

    // Get current agent.
    let agent = repo::agents::get_for_chat(&mut *tx, chat_id).await?;
    let agent_abilities = repo::abilities::list_for_agent(&mut *tx, agent.id).await?;
    let abilities = match abilities {
        Some(abilities) => abilities.into_iter().chain(agent_abilities).collect(),
        None => agent_abilities,
    };

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

    let api_key = match settings_guard
        .api_keys
        .get(&model.provider)
        .with_context(|| format!("Failed to get api key for provider: {:?}", model.provider))
    {
        Ok(api_key) => api_key,
        Err(err) => {
            fail_message(&window, &pool, &mut message).await?;

            return Err(err.into());
        }
    };

    // Send request to LLM
    let client = Client::new(api_key, model.api_url_or_default());
    let mut response = match client
        .create_chat_completion_stream(CreateChatCompletionRequest {
            model: &model.name,
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
