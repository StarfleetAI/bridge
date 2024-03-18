// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use anyhow::{anyhow, Context};
use serde_json::json;
use tauri::{AppHandle, Manager, State};
use tokio::spawn;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tracing::{debug, error, info, instrument, trace};

use crate::clients::openai::ToolCall;
use crate::repo::abilities::Ability;
use crate::repo::chats::{Chat, Kind};
use crate::repo::messages::{CreateParams, Message, Role};
use crate::repo::tasks::{Status, Task};
use crate::repo::{self};
use crate::settings::Settings;
use crate::types::{DbPool, Result};
use crate::{chats, errors};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no root tasks to execute")]
    NoRootTasks,
    #[error("chat #{0} is not an execution chat")]
    NotAnExecutionChat(i64),
}

// TODO: implement graceful shutdown
#[instrument(skip(app_handle))]
pub async fn start_loop(app_handle: AppHandle) {
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
    let window = app_handle
        .get_window("main")
        .context("failed to get main window")?;
    let pool: State<'_, DbPool> = app_handle.state();

    let mut task = match get_task_for_execution(&pool, None).await {
        Ok(Some(task)) => task,
        Ok(None) => return Err(Error::NoRootTasks.into()),
        Err(err) => return Err(err),
    };

    window
        .emit_all("tasks:updated", &task)
        .context("Failed to emit event")?;

    info!("Root task for execution: #{}. {}", task.id, task.title);

    match execute_task(app_handle, &mut task).await {
        Ok(status) => {
            debug!(
                "No errors. Transitioning root task #{} to status: {:?}",
                task.id, status
            );
            repo::tasks::update_status(&*pool, task.id, status).await?;

            task.status = status;
            window
                .emit_all("tasks:updated", &task)
                .context("Failed to emit event")?;

            Ok(())
        }
        Err(err) => {
            repo::tasks::fail(&*pool, task.id).await?;

            task.status = Status::Failed;
            window
                .emit_all("tasks:updated", &task)
                .context("Failed to emit event")?;

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
    let window = app_handle
        .get_window("main")
        .context("failed to get main window")?;
    window
        .emit_all("tasks:updated", &task)
        .context("Failed to emit event")?;

    loop {
        match repo::messages::get_last_message(&*pool, chat.id).await? {
            Some(message) => match message.role {
                Role::Tool | Role::User => send_to_agent(chat.id, app_handle, task).await?,
                Role::Assistant => match &message.tool_calls {
                    Some(tool_calls) => {
                        // I acknowledge, that this is weird to pass `tool_calls` alongside the `message`, but why not since it's already unpacked from `Option`?
                        if let Some(new_status) =
                            call_tools(&message, app_handle, tool_calls).await?
                        {
                            return Ok(new_status);
                        }
                    }
                    None => {
                        // TODO: it's better to send a task and a last message to a "router" LLM
                        //       call to get a response with a function call
                        repo::messages::create(
                            &*pool,
                            CreateParams {
                                chat_id: chat.id,
                                role: Role::User,
                                status: repo::messages::Status::Completed,
                                content: Some(
                                    "Respond only with a tool call according to a situation"
                                        .to_string(),
                                ),
                                ..Default::default()
                            },
                        )
                        .await?;

                        send_to_agent(chat.id, app_handle, task).await?;
                    }
                },
                Role::System => {
                    return Err(anyhow!("unexpected system message in the execution chat").into());
                }
            },
            None => send_to_agent(chat.id, app_handle, task).await?,
        }
    }
}

/// Call tools.
///
/// Returns optional new task status. This is useful when the task execution is finished and the
/// task status should be updated. For example, when the LLM marks the task as `Completed`.
#[instrument(skip(message, app_handle, tool_calls))]
async fn call_tools(
    message: &Message,
    app_handle: &AppHandle,
    tool_calls: &str,
) -> Result<Option<Status>> {
    let mut new_status = None;
    let tool_calls: Vec<ToolCall> =
        serde_json::from_str(tool_calls).context("failed to parse tool calls")?;

    // Call task management tools
    for tool_call in tool_calls {
        new_status = match tool_call.function.name.as_str() {
            "sfai_done" => Some(Status::Done),
            "sfai_fail" => Some(Status::Failed),
            "sfai_wait_for_user" => Some(Status::WaitingForUser),
            _ => None,
        }
    }

    // Call other tools
    crate::abilities::execute_for_message(message, app_handle).await?;

    Ok(new_status)
}

const BRIDGE_AGENT_PROMPT: &str =
    "Call `sfai_done` once you think you've completed the task, `sfai_fail` if you think the task is not possible to complete, or `sfai_wait_for_user` if you need more information from the user.";

#[instrument(skip(task, app_handle))]
async fn send_to_agent(chat_id: i64, app_handle: &AppHandle, task: &Task) -> Result<()> {
    let pool: State<'_, DbPool> = app_handle.state();
    let agent = repo::agents::get_for_chat(&*pool, chat_id).await?;

    let system_prompt = format!("{}\n\n---\n\n{}", agent.system_message, BRIDGE_AGENT_PROMPT);

    let messages = vec![
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
    ];

    // TODO: it's inefficient to use `Ability` here, since we're serializing parameters to JSON
    //       only to deserialize them back in `chats::get_completion`. Consider using [`Tool`]
    //       instead.
    //
    // TODO: It's also slightly inefficient to create these abilities on every iteration.
    //       Consider caching them or something.
    //
    // TODO: `description` is not sent to the OpenAI right now.
    let abilities = vec![
        Ability::for_fn(
            "Mark task as Done",
            "Mark current task as done",
            &json!({
                "name": "sfai_done",
                "description": "Mark current task as done."
            }),
        ),
        Ability::for_fn(
            "Mark task as Failed",
            "Mark current task as failed",
            &json!({
                "name": "sfai_fail",
                "description": "Mark current task as failed."
            }),
        ),
        Ability::for_fn(
            "Wait for user",
            "Wait for user input",
            &json!({
                "name": "sfai_wait_for_user",
                "description": "Wait for user input."
            }),
        ),
    ];

    // {"name":"read_file","parameters":{"type":"object","properties":{"file_name":{"type":"string","description":"File name to read"}}}}

    chats::get_completion(chat_id, app_handle, Some(messages), Some(abilities)).await?;

    Ok(())
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

    let task = match parent {
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

    tx.commit().await.context("failed to commit transaction")?;

    Ok(Some(task))
}
