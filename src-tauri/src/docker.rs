// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0
use std::collections::HashMap;
use std::path::Path;

use anyhow::Context;
use bollard::models::{ContainerInspectResponse, PortBinding};
use bollard::{
    container::{Config, RemoveContainerOptions},
    exec::{CreateExecOptions, StartExecResults},
    image::CreateImageOptions,
    secret::HostConfig,
};
use futures_util::{StreamExt, TryStreamExt};
use tokio::sync::OnceCell;
use tracing::trace;

use crate::types::Result;

const CONTAINER_WORKDIR: &str = "/bridge";
const DEFAULT_PYTHON_IMAGE: &str = "python:slim";
const DEFAULT_CHROMEDRIVER_IMAGE: &str = "zenika/alpine-chrome:with-chromedriver";

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
/// TODO move to `ContainerManager`
pub async fn run_python_code(script: &str, maybe_workdir: Option<&Path>) -> Result<String> {
    let binds = binds_for(maybe_workdir);
    let cmd = vec!["python", "-c", &script];

    run_in_container(DEFAULT_PYTHON_IMAGE, binds, cmd).await
}

/// Run a Python script in a container.
///
/// # Errors
///
/// Will return an error if there was a problem while running the script.
/// TODO move to `ContainerManager`
pub async fn run_python_script(workdir: &Path, script_name: &str) -> Result<String> {
    let binds = binds_for(Some(workdir));
    let script_name = format!("{CONTAINER_WORKDIR}/{script_name}");
    let cmd = vec!["python", &script_name];

    run_in_container(DEFAULT_PYTHON_IMAGE, binds, cmd).await
}

/// Run a shell command in a container.
///
/// # Errors
///
/// Will return an error if there was a problem while running the command.
pub async fn run_cmd(cmd: &str, maybe_workdir: Option<&Path>) -> Result<String> {
    let binds = binds_for(maybe_workdir);
    let cmd = vec!["sh", "-c", cmd];

    run_in_container(DEFAULT_PYTHON_IMAGE, binds, cmd).await
}

/// TODO move to `ContainerManager`
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

/// Сentrally manages containers.
pub struct ContainerManager {
    /// The docker client
    client: bollard::Docker,
}

static CONTAINER_MANAGER: OnceCell<ContainerManager> = OnceCell::const_new();

impl ContainerManager {
    /// Initialises the docker client.
    ///
    /// # Errors
    ///
    /// Will return an error if there was a problem while initialising the docker client.
    pub async fn get() -> Result<&'static Self> {
        CONTAINER_MANAGER
            .get_or_try_init(|| async {
                Ok(ContainerManager {
                    client: bollard::Docker::connect_with_local_defaults()
                        .map_err(Error::Bollard)?,
                })
            })
            .await
    }

    /// Function for starting chromedriver container.
    ///
    /// # Errors
    ///
    /// Will return an error if there was a problem while starting the chromedriver container.
    pub async fn launch_chromedriver_container(&self) -> Result<String> {
        let container_config = Config {
            image: Some(DEFAULT_CHROMEDRIVER_IMAGE),
            tty: Some(true),
            host_config: Some(HostConfig {
                auto_remove: Some(true),
                port_bindings: {
                    let mut map = HashMap::with_capacity(1);
                    map.insert(
                        "9515/tcp".to_string(),
                        Some(vec![PortBinding {
                            host_ip: None,
                            host_port: Some(String::new()),
                        }]),
                    );
                    Some(map)
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let container_id = self
            .client
            .create_container::<&str, &str>(None, container_config)
            .await
            .map_err(Error::Bollard)?
            .id;

        self.client
            .start_container::<String>(&container_id, None)
            .await
            .map_err(Error::Bollard)?;

        Ok(container_id)
    }

    /// Get container information.
    ///
    /// # Errors
    ///
    /// Will return an error if there was a problem while getting the container information.
    pub async fn inspect_container(&self, container_id: &str) -> Result<ContainerInspectResponse> {
        let container_info = self
            .client
            .inspect_container(container_id, None)
            .await
            .map_err(Error::Bollard)?;
        Ok(container_info)
    }

    /// Destroys the container.
    ///
    /// # Errors
    ///
    /// Will return an error if there was a problem while destroying the container.
    pub async fn kill_container(&self, container_name: &str) -> Result<()> {
        self.client
            .kill_container::<String>(container_name, None)
            .await
            .map_err(Error::Bollard)?;
        Ok(())
    }
}
