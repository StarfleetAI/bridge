use std::path::PathBuf;

use anyhow::Context;
use askama::Template;
use log::{debug, trace};
use tokio::fs;
use tokio::fs::create_dir_all;
use tokio::process::Command;

use crate::clients::openai::ToolCall;
use crate::repo::abilities::Ability;
use crate::repo::messages::Message;
use crate::types::Result;

#[derive(Template)]
#[template(path = "python/call_tools.py", escape = "none")]
struct CallToolsTemplate<'a> {
    code: &'a str,
    python_path: &'a str,
    tool_call: &'a str,
}

/// Execute abilities code.
///
/// # Errors
///
/// Will return an error if the script can't be written, executed or removed.
pub async fn execute(
    abilities: Vec<Ability>,
    app_local_data_dir: PathBuf,
    message: Message,
    tool_call: ToolCall,
    python_path: String,
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
        python_path: &python_path,
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
    let output = Command::new(&python_path)
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
