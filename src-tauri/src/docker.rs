// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use anyhow::Context;
use bollard::{
    container::{Config, RemoveContainerOptions},
    exec::{CreateExecOptions, StartExecResults},
    image::CreateImageOptions,
    secret::HostConfig,
};
use futures_util::{StreamExt, TryStreamExt};
use tracing::trace;

use crate::types::Result;

const CONTAINER_WORKDIR: &str = "/bridge";
const DEFAULT_IMAGE: &str = "python:slim";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Bollard(#[from] bollard::errors::Error),
}

/// Run a Python code in a container.
///
/// # Errors
///
/// Will return an error if there was a problem while running the code.
pub async fn run_python_code(script: &str, maybe_workdir: Option<&Path>) -> Result<String> {
    let binds = binds_for(maybe_workdir);
    let cmd = vec!["python", "-c", &script];

    run_in_container(DEFAULT_IMAGE, binds, cmd).await
}

/// Run a Python script in a container.
///
/// # Errors
///
/// Will return an error if there was a problem while running the script.
pub async fn run_python_script(workdir: &Path, script_name: &str) -> Result<String> {
    let binds = binds_for(Some(workdir));
    let script_name = format!("{CONTAINER_WORKDIR}/{script_name}");
    let cmd = vec!["python", &script_name];

    run_in_container(DEFAULT_IMAGE, binds, cmd).await
}

/// Run a shell command in a container.
///
/// # Errors
///
/// Will return an error if there was a problem while running the command.
pub async fn run_cmd(cmd: &str, maybe_workdir: Option<&Path>) -> Result<String> {
    let binds = binds_for(maybe_workdir);
    let cmd = vec!["sh", "-c", cmd];

    run_in_container(DEFAULT_IMAGE, binds, cmd).await
}

async fn run_in_container(
    image: &str,
    binds: Option<Vec<String>>,
    cmd: Vec<&str>,
) -> Result<String> {
    let docker = bollard::Docker::connect_with_local_defaults().map_err(Error::Bollard)?;

    docker
        .create_image(
            Some(CreateImageOptions {
                from_image: image,
                ..Default::default()
            }),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .context("Failed to create image")?;

    let config = Config {
        image: Some(image),
        tty: Some(true),
        host_config: Some(HostConfig {
            binds,
            auto_remove: Some(true),
            ..Default::default()
        }),
        ..Default::default()
    };

    let id = docker
        .create_container::<&str, &str>(None, config)
        .await
        .map_err(Error::Bollard)?
        .id;

    docker
        .start_container::<String>(&id, None)
        .await
        .map_err(Error::Bollard)?;

    let mut out = String::new();

    let exec = docker
        .create_exec(
            &id,
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(cmd),
                working_dir: Some(CONTAINER_WORKDIR),
                ..Default::default()
            },
        )
        .await
        .map_err(Error::Bollard)?
        .id;

    if let StartExecResults::Attached { mut output, .. } = docker
        .start_exec(&exec, None)
        .await
        .map_err(Error::Bollard)?
    {
        while let Some(Ok(msg)) = output.next().await {
            out.push_str(&msg.to_string());
        }
    }

    docker
        .remove_container(
            &id,
            Some(RemoveContainerOptions {
                force: true,
                ..Default::default()
            }),
        )
        .await
        .map_err(Error::Bollard)?;

    out = out.trim().to_string();

    trace!("Script output: {:?}", out);

    Ok(out.to_string())
}

fn binds_for(maybe_workdir: Option<&Path>) -> Option<Vec<String>> {
    maybe_workdir.map(|workdir| vec![format!("{}:{CONTAINER_WORKDIR}", workdir.to_string_lossy())])
}
