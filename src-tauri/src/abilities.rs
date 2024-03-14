// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use anyhow::{anyhow, Context};
use askama::Template;
use tauri::{AppHandle, Manager, State};
use tokio::fs::create_dir_all;
use tokio::process::Command;
use tokio::sync::RwLock;
use tokio::{fs, spawn};
use tracing::{debug, trace};

use crate::clients::openai::ToolCall;
use crate::repo;
use crate::repo::abilities::Ability;
use crate::repo::messages::{CreateParams, Message, Role, Status};
use crate::settings::Settings;
use crate::types::{DbPool, Result};

#[derive(Template)]
#[template(path = "python/call_tools.py", escape = "none")]
struct CallToolsTemplate<'a> {
    code: &'a str,
    python_path: &'a str,
    tool_call: &'a str,
}

/// Executes tool calls for the message.
///
/// # Errors
///
/// Will return an error if there was a problem while executing tool calls.
pub async fn execute_for_message(message: &Message, app_handle: &AppHandle) -> Result<()> {
    let pool: State<'_, DbPool> = app_handle.state();
    let settings: State<'_, RwLock<Settings>> = app_handle.state();

    let window = app_handle
        .get_window("main")
        .context("Failed to get main window")?;

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
            let output =
                execute(&abilities, &app_local_data_dir, &msg, &tc, &python_path_str).await?;
            // Wrap output in a code block
            //
            // TODO: This is a temporary solution. It's better to wrap it on before markdown-2-html
            //       processing, but it requires writing custom Serializer for Message.
            let output = format!("```\n{output}\n```");
            Ok::<_, anyhow::Error>(CreateParams {
                chat_id: msg.chat_id,
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

    Ok(())
}

/// Execute abilities code.
///
/// # Errors
///
/// Will return an error if the script can't be written, executed or removed.
pub async fn execute(
    abilities: &[Ability],
    app_local_data_dir: &PathBuf,
    message: &Message,
    tool_call: &ToolCall,
    python_path: &str,
) -> Result<String> {
    debug!(
        "Executing tool call `{}` for message `{}`",
        tool_call.id, message.id
    );
    // Join the abilities code into one string
    let code = abilities
        .iter()
        .map(|ability| ability.code.as_str())
        .collect::<Vec<&str>>()
        .join("\n\n");

    let workdir_name = format!("wd-{}", message.chat_id);

    // Build workdir path
    let mut workdir = PathBuf::new();
    workdir.push(app_local_data_dir);
    workdir.push(workdir_name);

    trace!("Workdir: {:?}", workdir);

    if !workdir.exists() {
        create_dir_all(&workdir)
            .await
            .with_context(|| "Failed to create workdir")?;
    }

    let tool_call_string =
        serde_json::to_string(&tool_call).with_context(|| "Failed to serialize tool call")?;

    let script_name = format!("tc-{}-{}.py", message.id, tool_call.id);
    let call_tools_template = CallToolsTemplate {
        code: &code,
        tool_call: &tool_call_string,
        python_path,
    };
    let content = call_tools_template
        .render()
        .with_context(|| "Failed to render `call_tools` script")?;

    trace!("Script name: {}", script_name);
    trace!("Script content: {}", content);

    // Write script to workdir
    let mut script_path = workdir.clone();
    script_path.push(script_name);
    trace!("Script path: {:?}", script_path);

    fs::write(&script_path, content)
        .await
        .with_context(|| "Failed to write script to workdir")?;

    // Run script
    let output = Command::new(python_path)
        .current_dir(&workdir)
        .arg(&script_path)
        .output()
        .await
        .with_context(|| "Failed to execute tool_calls script")?;

    trace!("Function call script output: {:?}", output);

    let output = if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
    } else {
        String::from_utf8_lossy(&output.stderr)
    };

    // Delete script
    fs::remove_file(&script_path)
        .await
        .with_context(|| "Failed to remove script from workdir")?;

    Ok(output.to_string())
}
