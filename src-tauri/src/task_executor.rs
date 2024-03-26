// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use anyhow::{anyhow, Context};
use serde::Deserialize;
use serde_json::json;
use tauri::{AppHandle, Manager, State};
use tokio::spawn;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tracing::{debug, error, info, instrument, trace};

use crate::clients::openai::ToolCall;
use crate::repo::{
    self,
    abilities::Ability,
    agents::Agent,
    chats::{Chat, Kind},
    messages::{CreateParams, Message, Role},
    tasks::{Status, Task},
};
use crate::settings::Settings;
use crate::types::{DbPool, Result};
use crate::{
    chats::{self, GetCompletionParams},
    errors, utils,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no root tasks to execute")]
    NoRootTasks,
    #[error("chat #{0} is not an execution chat")]
    NotAnExecutionChat(i64),
}

// TODO: implement graceful shutdown
#[instrument(skip(app_handle))]
pub async fn start_loop(app_handle: &AppHandle) {
    let settings_state: State<'_, RwLock<Settings>> = app_handle.state();
    let settings = settings_state.read().await;

    info!(
        "Starting task execution loop with concurrency = {}",
        settings.tasks.execution_concurrency
    );

    for i in 0..settings.tasks.execution_concurrency {
        let app_handle = app_handle.clone();

        spawn(async move {
            loop {
                if let Err(err) = execute_root_task(&app_handle).await {
                    if let errors::Error::Executor(Error::NoRootTasks) = err {
                        trace!("No root tasks to execute, waiting...");

                        sleep(Duration::from_secs(1)).await;
                    } else {
                        error!("Failed to execute task: {:?}", err);
                    }
                }
            }
        });

        debug!("-- Thread #{} started", i);
    }
}

#[instrument(skip(app_handle))]
async fn execute_root_task(app_handle: &AppHandle) -> Result<()> {
    let pool: State<'_, DbPool> = app_handle.state();

    let mut task = match get_task_for_execution(&pool, None).await {
        Ok(Some(task)) => task,
        Ok(None) => return Err(Error::NoRootTasks.into()),
        Err(err) => return Err(err),
    };

    utils::emit_event(app_handle, "tasks:updated", &task)?;

    info!("Root task for execution: #{}. {}", task.id, task.title);

    match execute_task(app_handle, &mut task).await {
        Ok(status) => {
            debug!(
                "No errors. Transitioning root task #{} to status: {:?}",
                task.id, status
            );
            repo::tasks::update_status(&*pool, task.id, status).await?;

            task.status = status;
            utils::emit_event(app_handle, "tasks:updated", &task)?;

            Ok(())
        }
        Err(err) => {
            repo::tasks::fail(&*pool, task.id).await?;

            task.status = Status::Failed;
            utils::emit_event(app_handle, "tasks:updated", &task)?;

            return Err(err);
        }
    }

    // while let Some(child) = match get_task_for_execution(&*pool, Some(&task)).await {
    //     Ok(task) => task,
    //     Err(err) => {
    //         tasks::fail(&*pool, task.id).await?;
    //
    //         bail!(err)
    //     }
    // } {
    //     match execute_task(&app_handle, &child).await {
    //         Ok(_) => {
    //             tasks::complete(&*pool, child.id).await?;
    //         }
    //         Err(err) => {
    //             tasks::fail(&*pool, child.id).await?;
    //             tasks::fail(&*pool, task.id).await?;
    //
    //             bail!(err.context("failed to execute child task"))
    //         }
    //     };
    // }
}

#[instrument(skip(app_handle, task))]
async fn execute_task(app_handle: &AppHandle, task: &mut Task) -> Result<Status> {
    info!("Executing task #{}: {}", task.id, task.title);

    let pool: State<'_, DbPool> = app_handle.state();
    let chat = get_task_execution_chat(&pool, task).await?;

    task.execution_chat_id = Some(chat.id);
    utils::emit_event(app_handle, "tasks:updated", &task)?;

    // TODO: refactor this loop
    loop {
        match repo::messages::get_last_message(&*pool, chat.id).await? {
            Some(message) => match message.role {
                Role::Tool | Role::User => send_to_agent(chat.id, app_handle, task).await?,
                Role::Assistant => {
                    match &message.tool_calls {
                        Some(tool_calls) => {
                            // I acknowledge, that this is weird to pass `tool_calls` alongside the `message`, but why not since it's already unpacked from `Option`?
                            match call_tools(&message, app_handle, tool_calls, task.id).await {
                                Ok(maybe_new_status) => {
                                    complete_message(&message, app_handle).await?;

                                    if let Some(new_status) = maybe_new_status {
                                        return Ok(new_status);
                                    }
                                }
                                Err(err) => {
                                    fail_message(&message, app_handle).await?;
                                    return Err(err);
                                }
                            }
                        }
                        None if message.is_self_reflection => {
                            send_to_agent(chat.id, app_handle, task).await?;
                        }
                        None => self_reflect(chat.id, app_handle, task).await?,
                    }
                }
                Role::System => {
                    return Err(anyhow!("unexpected system message in the execution chat").into());
                }
            },
            None => send_to_agent(chat.id, app_handle, task).await?,
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct ProvideTextResultArgs {
    pub text: String,
    pub is_done: bool,
}

/// Call tools.
///
/// Returns optional new task status. This is useful when the task execution is finished and the
/// task status should be updated. For example, when the LLM marks the task as `Done`.
#[instrument(skip(message, app_handle, tool_calls))]
async fn call_tools(
    message: &Message,
    app_handle: &AppHandle,
    tool_calls: &str,
    task_id: i64,
) -> Result<Option<Status>> {
    let mut new_status = None;

    let tool_calls: Vec<ToolCall> =
        serde_json::from_str(tool_calls).context("failed to parse tool calls")?;

    // Call task management tools
    for tool_call in tool_calls {
        if let Some(status) = match tool_call.function.name.as_str() {
            "sfai_done" => sfai_done(message, app_handle, task_id, &tool_call).await?,
            "sfai_fail" => Some(Status::Failed),
            "sfai_wait_for_user" => Some(Status::WaitingForUser),
            "sfai_provide_text_result" => {
                sfai_provide_text_result(
                    message,
                    app_handle,
                    task_id,
                    tool_call.id.clone(),
                    serde_json::from_str(&tool_call.function.arguments).context(
                        "failed to parse tool call arguments for `sfai_provide_text_result`",
                    )?,
                )
                .await?
            }
            _ => None,
        } {
            new_status = Some(status);
        }
    }

    // Call other tools
    crate::abilities::execute_for_message(message, app_handle).await?;

    Ok(new_status)
}

async fn sfai_done(
    message: &Message,
    app_handle: &AppHandle,
    task_id: i64,
    tool_call: &ToolCall,
) -> Result<Option<Status>> {
    let pool: State<'_, DbPool> = app_handle.state();

    if let Some(result_message) =
        repo::messages::get_last_non_self_reflection_message(&*pool, message.chat_id).await?
    {
        let text = result_message.content.clone().unwrap_or_default();

        sfai_provide_text_result(
            &result_message,
            app_handle,
            task_id,
            tool_call.id.clone(),
            ProvideTextResultArgs {
                text,
                ..Default::default()
            },
        )
        .await?;
    }

    Ok(Some(Status::Done))
}

/// Provide a text result for the task.
///
/// # Errors
///
/// Returns an error if the tool call arguments cannot be parsed or the task result cannot be
/// created.
#[instrument(skip(message, app_handle, task_id, tool_call_id, args))]
async fn sfai_provide_text_result(
    message: &Message,
    app_handle: &AppHandle,
    task_id: i64,
    tool_call_id: String,
    args: ProvideTextResultArgs,
) -> Result<Option<Status>> {
    let mut new_status = None;
    let pool: State<'_, DbPool> = app_handle.state();

    let task_result = repo::task_results::create(
        &*pool,
        repo::task_results::CreateParams {
            agent_id: message
                .agent_id
                .context("Agent is not set for the message with a tool call")?,
            task_id,
            kind: repo::task_results::Kind::Text,
            data: args.text,
        },
    )
    .await?;

    utils::emit_event(app_handle, "task_results:created", &task_result)?;

    if args.is_done {
        new_status = Some(Status::Done);
    }

    let message = repo::messages::create(
        &*pool,
        CreateParams {
            chat_id: message.chat_id,
            status: repo::messages::Status::Completed,
            role: Role::Tool,
            content: Some("Text result has been created".to_string()),
            tool_call_id: Some(tool_call_id),
            is_internal_tool_output: true,
            ..Default::default()
        },
    )
    .await?;

    utils::emit_event(app_handle, "messages:created", &message)?;

    Ok(new_status)
}

async fn complete_message(message: &Message, app_handle: &AppHandle) -> Result<()> {
    let pool: State<'_, DbPool> = app_handle.state();
    repo::messages::update_status(&*pool, message.id, repo::messages::Status::Completed).await?;

    let mut message = message.clone();
    message.status = repo::messages::Status::Completed;

    utils::emit_event(app_handle, "messages:updated", &message)?;

    Ok(())
}

async fn fail_message(message: &Message, app_handle: &AppHandle) -> Result<()> {
    let pool: State<'_, DbPool> = app_handle.state();
    repo::messages::update_status(&*pool, message.id, repo::messages::Status::Failed).await?;

    let mut message = message.clone();
    message.status = repo::messages::Status::Failed;

    utils::emit_event(app_handle, "messages:updated", &message)?;

    Ok(())
}

const BRIDGE_AGENT_PROMPT: &str = r"As you work on a task assigned by a user, strive to complete it with your best effort and deliver the outcome to the user.

If, due to technical issues or other reasons, you are unable to provide the result, you have the option to mark the task as failed.
Should you require further information from the user, feel free to request it.";

const SELF_REFLECTION_PROMPT: &str = r"Conduct an internal reflection on your message.

If the response aligns with what the user expects, proceed to use the `sfai_done` tool.
In cases where the response appears incorrect or doesn't meet the user's requirements, articulate your reasoning aloud and determine how to enhance the answer.

Should technical or other issues prevent providing a result, designate the task as unsuccessful using the `sfai_fail` tool.
If further information from the user is required, request it and utilize the `sfai_wait_for_user` tool.";

#[instrument(skip(task, app_handle))]
async fn send_to_agent(chat_id: i64, app_handle: &AppHandle, task: &Task) -> Result<()> {
    let pool: State<'_, DbPool> = app_handle.state();
    let agent = repo::agents::get_for_chat(&*pool, chat_id).await?;

    chats::get_completion(
        app_handle,
        chat_id,
        GetCompletionParams {
            messages_pre: Some(execution_prelude(chat_id, task, &agent)),
            ..Default::default()
        },
    )
    .await?;

    Ok(())
}

#[instrument(skip(task, app_handle))]
async fn self_reflect(chat_id: i64, app_handle: &AppHandle, task: &Task) -> Result<()> {
    let pool: State<'_, DbPool> = app_handle.state();
    let agent = repo::agents::get_for_chat(&*pool, chat_id).await?;

    let messages_post = vec![Message {
        chat_id,
        role: Role::User,
        content: Some(SELF_REFLECTION_PROMPT.to_string()),
        ..Default::default()
    }];

    chats::get_completion(
        app_handle,
        chat_id,
        GetCompletionParams {
            messages_pre: Some(execution_prelude(chat_id, task, &agent)),
            messages_post: Some(messages_post),
            abilities: Some(internal_task_abilities()),
            is_self_reflection: true,
        },
    )
    .await?;

    Ok(())
}

fn internal_task_abilities() -> Vec<Ability> {
    // TODO: it's inefficient to use `Ability` here, since we're serializing parameters to JSON
    //       only to deserialize them back in `chats::get_completion`. Consider using [`Tool`]
    //       instead.
    //
    // TODO: It's also slightly inefficient to create these abilities on every iteration.
    //       Consider caching them or something.
    vec![
        Ability::for_fn(
            "Mark current task as done",
            &json!({
                "name": "sfai_done",
            }),
        ),
        Ability::for_fn(
            "Mark current task as failed",
            &json!({
                "name": "sfai_fail",
            }),
        ),
        Ability::for_fn(
            "Wait for additional user input",
            &json!({
                "name": "sfai_wait_for_user",
            }),
        ),
    ]
}

fn execution_prelude(chat_id: i64, task: &Task, agent: &Agent) -> Vec<Message> {
    let system_prompt = format!("{}\n\n---\n\n{}", agent.system_message, BRIDGE_AGENT_PROMPT);

    vec![
        Message {
            chat_id,
            role: Role::System,
            content: Some(system_prompt),
            ..Default::default()
        },
        Message {
            chat_id,
            role: Role::User,
            content: Some(format!("# Task: {}\n\n{}", task.title, task.summary)),
            ..Default::default()
        },
    ]
}

#[instrument(skip(pool, task))]
async fn get_task_execution_chat(pool: &DbPool, task: &Task) -> Result<Chat> {
    if let Some(chat_id) = task.execution_chat_id {
        match repo::chats::get(pool, chat_id).await {
            Ok(chat) if chat.kind == Kind::Execution => Ok(chat),
            Ok(_) => Err(Error::NotAnExecutionChat(chat_id).into()),
            Err(err) => Err(err),
        }
    } else {
        let chat = repo::chats::create(pool, Kind::Execution).await?;
        repo::tasks::update_execution_chat_id(pool, task.id, chat.id).await?;
        repo::agents_chats::create(pool, task.agent_id, chat.id).await?;

        Ok(chat)
    }
}

#[instrument(skip(pool, parent))]
async fn get_task_for_execution(pool: &DbPool, parent: Option<&Task>) -> Result<Option<Task>> {
    let mut tx = pool.begin().await.context("failed to begin transaction")?;

    let mut task = match parent {
        Some(parent) => {
            if let Some(task) =
                repo::tasks::get_children_for_execution(&mut *tx, &parent.children_ancestry())
                    .await?
            {
                task
            } else {
                tx.commit().await.context("failed to commit transaction")?;

                return Ok(None);
            }
        }
        None => {
            if let Some(task) = repo::tasks::get_root_for_execution(&mut *tx).await? {
                task
            } else {
                tx.commit().await.context("failed to commit transaction")?;

                return Ok(None);
            }
        }
    };

    // Check if task is ready to be executed.
    //
    // Since sub-tasks execution is sequential, we want to catch the cases when there is a sub-task
    // that is not in `ToDo` status and stop the execution of the parent task.
    if task.status != Status::ToDo {
        tx.commit().await.context("failed to commit transaction")?;

        return Err(anyhow!("Task #{} is not in `ToDo` status", task.id).into());
    }

    repo::tasks::start_progress(&mut *tx, task.id).await?;
    task.status = Status::InProgress;

    tx.commit().await.context("failed to commit transaction")?;

    Ok(Some(task))
}
