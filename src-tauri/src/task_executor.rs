// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use anyhow::{anyhow, Context};
use serde::Deserialize;
use serde_json::json;
use tauri::{AppHandle, Manager, State};
use tokio::sync::RwLock;
use tokio::time::sleep;
use tokio::{fs, spawn};
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
    docker, errors,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no root tasks to execute")]
    NoRootTasks,
    #[error("chat #{0} is not an execution chat")]
    NotAnExecutionChat(i64),
    #[error("task execution steps limit ({0}) exceeded")]
    StepsLimitExceeded(i64),
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

    let mut task = match get_root_task_for_execution(&pool).await {
        Ok(Some(task)) => task,
        Ok(None) => return Err(Error::NoRootTasks.into()),
        Err(err) => return Err(err),
    };

    app_handle.emit_all("tasks:updated", &task)?;

    info!("Root task for execution: #{}. {}", task.id, task.title);

    let children_count = repo::tasks::get_all_children_count(&*pool, &task).await?;

    if children_count > 0 {
        info!("Executing children tasks for root task #{}.", task.id);
        execute_children_task_tree(app_handle, &mut task).await?;

        return Ok(());
    }

    info!("Executing root task #{}", task.id);

    match execute_task(app_handle, &mut task).await {
        Ok(status) => {
            debug!(
                "No errors. Transitioning root task #{} to status: {:?}",
                task.id, status
            );

            let task = repo::tasks::update_status(&*pool, task.id, status).await?;
            app_handle.emit_all("tasks:updated", &task)?;

            Ok(())
        }
        Err(err) => {
            let task = repo::tasks::fail(&*pool, task.id).await?;
            app_handle.emit_all("tasks:updated", &task)?;

            Err(err)
        }
    }
}

async fn execute_children_task_tree(app_handle: &AppHandle, parent: &mut Task) -> Result<()> {
    let pool: State<'_, DbPool> = app_handle.state();

    info!("Executing children tasks tree for task #{}", parent.id);

    while let Some(mut child) = match get_child_task_for_execution(&pool, parent).await {
        Ok(task) => task,
        Err(err) => {
            repo::tasks::fail(&*pool, parent.id).await?;
            fail_parent_tasks(app_handle, pool, parent).await?;

            return Err(err);
        }
    } {
        info!("Executing child task #{}: {}", child.id, child.title);

        // TODO: seems counterintuitive to emit the task update here, since it was updated in the
        //       `get_child_task_for_execution` function. Consider code reorganization.
        app_handle.emit_all("tasks:updated", &child)?;

        match execute_task(app_handle, &mut child).await {
            Ok(_) => {
                info!("Child task #{} is done", child.id);
                repo::tasks::complete(&*pool, child.id).await?;

                // Complete parent task if all siblings are done
                if repo::tasks::is_all_siblings_done(&*pool, &child).await? {
                    info!(
                        "All siblings are done for the parent task #{}, marking it as `Done` as well",
                        parent.id
                    );

                    let task = repo::tasks::complete(
                        &*pool,
                        child
                            .parent_id()?
                            .context("parent_id is not set for the child task")?,
                    )
                    .await?;

                    app_handle.emit_all("tasks:updated", &task)?;
                }
            }
            Err(err) => {
                repo::tasks::fail(&*pool, child.id).await?;
                fail_parent_tasks(app_handle, pool, &child).await?;

                return Err(err);
            }
        }
    }

    Ok(())
}

async fn fail_parent_tasks(
    app_handle: &AppHandle,
    pool: State<'_, DbPool>,
    child: &Task,
) -> Result<()> {
    if let Some(parent_ids) = child.parent_ids()? {
        for parent_id in parent_ids {
            let task = repo::tasks::fail(&*pool, parent_id).await?;
            app_handle.emit_all("tasks:updated", &task)?;
        }
    }

    Ok(())
}

#[instrument(skip(app_handle, task))]
async fn execute_task(app_handle: &AppHandle, task: &mut Task) -> Result<Status> {
    info!("Executing task #{}: {}", task.id, task.title);

    let pool: State<'_, DbPool> = app_handle.state();
    let chat = get_task_execution_chat(&pool, task).await?;

    task.execution_chat_id = Some(chat.id);
    app_handle.emit_all("tasks:updated", &task)?;

    let execution_steps_limit = {
        let agent = repo::agents::get_for_chat(&*pool, chat.id).await?;
        let settings_state: State<'_, RwLock<Settings>> = app_handle.state();
        let settings = settings_state.read().await;

        agent
            .execution_steps_limit
            .unwrap_or(settings.agents.execution_steps_limit)
    };

    loop {
        let messages_count = repo::messages::get_execution_steps_count(&*pool, chat.id).await?;
        if messages_count >= execution_steps_limit {
            return Err(Error::StepsLimitExceeded(execution_steps_limit).into());
        }
        match repo::messages::get_last_message(&*pool, chat.id).await? {
            Some(message) => match message.role {
                Role::CodeInterpreter | Role::Tool | Role::User => {
                    send_to_agent(chat.id, app_handle, task).await?;
                }
                Role::Assistant => {
                    match &message.tool_calls {
                        Some(tool_calls) => {
                            // I acknowledge, that this is weird to pass `tool_calls` alongside the `message`, but why not since it's already unpacked from `Option`?
                            match call_tools(&message, app_handle, tool_calls, task).await {
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
                        None => {
                            let content = message.content.clone().unwrap_or_default();
                            match parse_code_blocks(&content) {
                                Ok(code_blocks) if !code_blocks.is_empty() => {
                                    sfai_code_interpreter(app_handle, &message, task).await?;
                                }
                                _ => self_reflect(chat.id, app_handle, task).await?,
                            }
                        }
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
    task: &Task,
) -> Result<Option<Status>> {
    let mut new_status = None;

    let tool_calls: Vec<ToolCall> =
        serde_json::from_str(tool_calls).context("failed to parse tool calls")?;

    // Call task management tools
    for tool_call in tool_calls {
        if let Some(status) = match tool_call.function.name.as_str() {
            "sfai_done" => sfai_done(message, app_handle, task.id, &tool_call).await?,
            "sfai_fail" => Some(Status::Failed),
            "sfai_wait_for_user" => Some(Status::WaitingForUser),
            "sfai_provide_text_result" => {
                sfai_provide_text_result(
                    message,
                    app_handle,
                    task.id,
                    tool_call.id.clone(),
                    serde_json::from_str(&tool_call.function.arguments).context(
                        "failed to parse tool call arguments for `sfai_provide_text_result`",
                    )?,
                )
                .await?
            }
            "sfai_code_interpreter" => sfai_code_interpreter(app_handle, message, task).await?,
            _ => None,
        } {
            new_status = Some(status);
        }
    }

    // Call other tools
    crate::abilities::execute_for_message(message, app_handle).await?;

    Ok(new_status)
}

async fn sfai_code_interpreter(
    app_handle: &AppHandle,
    message: &Message,
    task: &Task,
) -> Result<Option<Status>> {
    let pool: State<'_, DbPool> = app_handle.state();

    if let Some(result_message) =
        repo::messages::get_last_non_self_reflection_message(&*pool, message.chat_id).await?
    {
        let content = Some(
            match interpret_code(app_handle, &result_message, task).await {
                Ok(out_lines) => out_lines.join("\n\n"),
                Err(err) => format!("Failed to interpret code: {err}"),
            },
        );

        let out_message = repo::messages::create(
            &*pool,
            CreateParams {
                content,
                chat_id: message.chat_id,
                status: repo::messages::Status::Completed,
                role: Role::CodeInterpreter,
                ..Default::default()
            },
        )
        .await?;

        app_handle.emit_all("messages:created", &out_message)?;
    }

    Ok(None)
}

async fn interpret_code(
    app_handle: &AppHandle,
    message: &Message,
    task: &Task,
) -> Result<Vec<String>> {
    let code_blocks = match parse_code_blocks(match &message.content.as_ref() {
        Some(content) => content,
        None => return Ok(vec!["No content in the message to interpret".to_string()]),
    }) {
        Ok(code_blocks) => code_blocks,
        Err(err) => {
            return Ok(vec![format!(
                "Failed to parse code blocks in the message: {err}"
            )]);
        }
    };

    let mut lines = Vec::with_capacity(code_blocks.len());

    let workdir = task.workdir(app_handle).await?;

    for code_block in code_blocks {
        if code_block.filename.is_none() {
            let result = match code_block.language {
                Language::Shell => docker::run_cmd(&code_block.code, Some(&workdir)).await?,
                Language::Python => {
                    docker::run_python_code(&code_block.code, Some(&workdir)).await?
                }
                lang => format!("Error: language `{lang:?}` is not supported for code execution"),
            };

            lines.push(format!("```\n{result}\n```"));
        } else if let Some(filename) = &code_block.filename {
            let mut workdir = match task.workdir(app_handle).await {
                Ok(workdir) => workdir,
                Err(err) => {
                    lines.push(format!("```\nFailed to get task workdir: {err}\n```"));
                    continue;
                }
            };

            workdir.push(filename);

            match fs::write(&workdir, code_block.code).await {
                Ok(()) => {
                    lines.push(format!("```\nFile `{filename}` has been saved\n```"));
                }
                Err(err) => {
                    lines.push(format!("```\nFailed to save file `{filename}`: {err}\n```"));
                }
            }
        }
    }

    Ok(lines)
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

    app_handle.emit_all("task_results:created", &task_result)?;

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

    app_handle.emit_all("messages:created", &message)?;

    Ok(new_status)
}

async fn complete_message(message: &Message, app_handle: &AppHandle) -> Result<()> {
    let pool: State<'_, DbPool> = app_handle.state();
    repo::messages::update_status(&*pool, message.id, repo::messages::Status::Completed).await?;

    let mut message = message.clone();
    message.status = repo::messages::Status::Completed;

    app_handle.emit_all("messages:updated", &message)?;

    Ok(())
}

async fn fail_message(message: &Message, app_handle: &AppHandle) -> Result<()> {
    let pool: State<'_, DbPool> = app_handle.state();
    repo::messages::update_status(&*pool, message.id, repo::messages::Status::Failed).await?;

    let mut message = message.clone();
    message.status = repo::messages::Status::Failed;

    app_handle.emit_all("messages:updated", &message)?;

    Ok(())
}

const BRIDGE_AGENT_PROMPT: &str = r"As you work on a task assigned by a user, strive to complete it with your best effort and deliver the outcome to the user.

If, due to technical issues or other reasons, you are unable to provide the result, you have the option to mark the task as failed.
Should you require further information from the user, feel free to request it.";

const SELF_REFLECTION_PROMPT: &str = r"Conduct an internal reflection on your message.

If the response aligns with what the user expects as a result, proceed to use the `sfai_done` tool.
In cases where the response appears incorrect or doesn't meet the user's requirements, articulate your reasoning aloud and determine how to enhance the answer.

Should technical or other issues prevent providing an exact result to user, designate the task as unsuccessful using the `sfai_fail` tool.
If further information from the user is required, request it and utilize the `sfai_wait_for_user` tool.

If the message you're reflecting on states that task execution is complete, you must use the `sfai_done` tool to mark the task as done, even if you've did it.
If the task execution result is not what user was asked for (for example, you weren't able to execute code or access web browser or any other technical issue occured), you must use the `sfai_fail` tool to mark the task as failed.";

const CODE_INTERPRETER_TOOL: &str = r#"### Code Interpreter Tool

You have access to a code interpreter. This allows you to:

- Save code snippets as files.
- Execute code snippets.
- Run bash commands.

#### Usage

You can prepend code blocks with the blockquote, containing either `Save: <filename>` or `Execute` to save or run the code respectively.

Examples:

> Execute
```python
print("This will be executed")
```

> Save: `my_script.py`
```python
print("Hello, World!")
```

> Save: `hello.sh`
```bash
echo "Hello, World!"
```

> Execute
```shell
python my_script.py
```

> Save: `README.md`
```markdown
Hello, World!
```

#### Notes

- Do not provide any explanations or additional text output while writing the code.
- You must indent any code blocks inside the generated Markdown documents by 4 spaces.
- Communicate the results from the code execution back to user, since he can't see the code execution output.
- If the execution of your code was failed because of the error in your code, you must do your best to fix the error by changing the relevant code."#;

#[instrument(skip(task, app_handle))]
async fn send_to_agent(chat_id: i64, app_handle: &AppHandle, task: &Task) -> Result<()> {
    let pool: State<'_, DbPool> = app_handle.state();
    let agent = repo::agents::get_for_chat(&*pool, chat_id).await?;

    chats::get_completion(
        app_handle,
        chat_id,
        GetCompletionParams {
            messages_pre: Some(execution_prelude(chat_id, task, &agent, false)),
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

    let content = Some(SELF_REFLECTION_PROMPT.to_string());

    let messages_post = vec![Message {
        chat_id,
        content,
        role: Role::User,
        ..Default::default()
    }];

    chats::get_completion(
        app_handle,
        chat_id,
        GetCompletionParams {
            messages_pre: Some(execution_prelude(chat_id, task, &agent, true)),
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

fn execution_prelude(
    chat_id: i64,
    task: &Task,
    agent: &Agent,
    is_self_reflection: bool,
) -> Vec<Message> {
    let mut system_lines = vec![&agent.system_message, BRIDGE_AGENT_PROMPT];

    if agent.is_code_interpreter_enabled && !is_self_reflection {
        system_lines.push(CODE_INTERPRETER_TOOL);
    }

    let system_prompt = system_lines.join("\n\n---\n\n");

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

#[instrument(skip(pool))]
async fn get_root_task_for_execution(pool: &DbPool) -> Result<Option<Task>> {
    let mut tx = pool.begin().await.context("failed to begin transaction")?;

    let Some(mut task) = repo::tasks::get_root_for_execution(&mut *tx).await? else {
        tx.commit().await.context("failed to commit transaction")?;

        return Ok(None);
    };

    if task.status != Status::ToDo {
        tx.commit().await.context("failed to commit transaction")?;

        return Ok(None);
    }

    repo::tasks::start_progress(&mut *tx, task.id).await?;
    task.status = Status::InProgress;

    tx.commit().await.context("failed to commit transaction")?;

    Ok(Some(task))
}

struct TaskTree {
    pub root: Task,
    pub children: Vec<TaskTree>,
}

#[instrument(skip(pool, parent))]
async fn get_child_task_for_execution(pool: &DbPool, parent: &Task) -> Result<Option<Task>> {
    let mut children_tasks = repo::tasks::list_all_children(pool, &parent.children_ancestry())
        .await
        .context("failed to list children")?;

    let mut tree = TaskTree {
        root: (*parent).clone(),
        children: Vec::new(),
    };

    sort_task_tree(&mut children_tasks);
    collect_children(&mut tree, &mut children_tasks)?;

    if let Some(task) = find_execution_candidate(&tree) {
        return Ok(Some(repo::tasks::start_progress(pool, task.id).await?));
    }

    Ok(None)
}

fn find_execution_candidate(tree: &TaskTree) -> Option<&Task> {
    if !tree.children.is_empty() {
        for child in &tree.children {
            if let Some(task) = find_execution_candidate(child) {
                return Some(task);
            }
        }
    }

    match tree.root.status {
        Status::InProgress | Status::Done => None,
        Status::Draft | Status::ToDo | Status::WaitingForUser | Status::Failed => Some(&tree.root),
    }
}

fn collect_children(tree: &mut TaskTree, tasks: &mut Vec<Task>) -> Result<()> {
    for task in tasks.clone() {
        if task.parent_id()? == Some(tree.root.id) {
            tree.children.push(TaskTree {
                root: task.clone(),
                children: Vec::new(),
            });

            tasks.retain(|t| t.id != task.id);

            collect_children(tree.children.last_mut().unwrap(), tasks)?;
        }
    }

    Ok(())
}

fn sort_task_tree(tasks: &mut [Task]) {
    tasks.sort_by(|a, b| a.created_at.cmp(&b.created_at));
}

#[derive(Default, Debug)]
enum Language {
    #[default]
    Unknown,
    Shell,
    Markdown,
    Python,
    Other,
}

impl From<String> for Language {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "sh" | "shell" => Language::Shell,
            "markdown" | "md" => Language::Markdown,
            "python" => Language::Python,
            "" => Language::Unknown,
            _ => Language::Other,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
enum CodeBlockAction {
    #[default]
    DoNothing,
    Execute,
    Save,
}

#[derive(Default)]
struct CodeBlock {
    pub code: String,
    pub language: Language,
    pub filename: Option<String>,
    pub action: CodeBlockAction,
}

fn parse_code_blocks(text: &str) -> Result<Vec<CodeBlock>> {
    let ast = markdown::to_mdast(text, &markdown::ParseOptions::default())
        .map_err(|err| anyhow!("Failed to parse markdown AST: {}", err))?;

    let mut code_blocks = Vec::new();
    let mut code_block = CodeBlock::default();

    for node in ast
        .children()
        .ok_or_else(|| anyhow!("Failed to get AST children"))?
    {
        match node {
            markdown::mdast::Node::BlockQuote(blockquote) => {
                if blockquote.children.len() != 1 {
                    continue;
                }

                let markdown::mdast::Node::Paragraph(paragraph) = &blockquote.children[0] else {
                    continue;
                };

                match paragraph.children.len() {
                    1 => {
                        if let markdown::mdast::Node::Text(text) = &paragraph.children[0] {
                            if text.value.to_lowercase().trim() != "execute" {
                                continue;
                            }

                            code_block.action = CodeBlockAction::Execute;
                        }
                    }
                    2 => {
                        if let markdown::mdast::Node::Text(text) = &paragraph.children[0] {
                            if text.value.to_lowercase().trim() != "save:" {
                                continue;
                            }

                            if let markdown::mdast::Node::InlineCode(ic) = &paragraph.children[1] {
                                code_block.filename = Some(ic.value.clone());
                                code_block.action = CodeBlockAction::Save;
                            }
                        }
                    }
                    _ => continue,
                }
            }
            markdown::mdast::Node::Code(code)
                if code_block.action != CodeBlockAction::DoNothing =>
            {
                code_block.code = code.value.clone();
                code_block.language = code.lang.clone().unwrap_or_default().into();
                code_blocks.push(code_block);
                code_block = CodeBlock::default();
            }
            _ => {}
        }
    }

    Ok(code_blocks)
}
