// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;

use bollard::{
    container::{Config, RemoveContainerOptions},
    exec::{CreateExecOptions, StartExecResults},
    secret::HostConfig,
};
use fantoccini::{Client, ClientBuilder, Locator, wd::Capabilities};
use crate::docker::ContainerManager;

use crate::types::Result;

const DEFAULT_CHROMEDRIVER_IMAGE: &str = "zenika/alpine-chrome:with-chromedriver";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to connect to WebDriver: {0}")]
    WebDriverConnection(#[from] fantoccini::error::NewSessionError),
    #[error("failed to execute WebDriver command: {0}")]
    WebDriverCmd(#[from] fantoccini::error::CmdError),
    #[error("failed to save screenshot: {0}")]
    ScreenshotSave(#[from] std::io::Error),
}

pub struct Browser {
    pub app_local_data_dir: String,
    pub client: Client,
    pub container_id: String,
    status: PhantomData<()>,
}

#[allow(clippy::module_name_repetitions)]
pub struct BrowserBuilder {
    app_local_data_dir: String,
}

impl BrowserBuilder {
    #[must_use]
    pub fn new(app_local_data_dir: String) -> Self {
        Self { app_local_data_dir }
    }

    /// Connect to `WebDriver`.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while connecting to `WebDriver`.
    pub async fn connect(self) -> anyhow::Result<Browser> {
        let mut caps = Capabilities::new();
        // TODO: support geckodriver
        let opts = serde_json::json!({
            "args": ["--headless", "--disable-gpu", "--no-sandbox", "--disable-dev-shm-usage"],
        });
        caps.insert("goog:chromeOptions".to_string(), opts);

        // TODO: launch a unique Docker container with chromedriver
        let docker_client = ContainerManager::new();
        let container_id = docker_client.launch_container(
            DEFAULT_CHROMEDRIVER_IMAGE,
            None,
            None
        ).await?;

        let container_info = docker_client.inspect_container(&container_id).await?;

        let container_port = container_info
            .network_settings
            .as_ref()
            .and_then(|network_settings| network_settings.ports.as_ref())
            .and_then(|ports| ports.get("9515/tcp"))
            .and_then(|port_bindings| port_bindings.as_ref())
            .and_then(|port_bindings| port_bindings.first())
            .and_then(|port_binding| port_binding.host_port.as_deref())
            .ok_or_else(|| {anyhow::anyhow!("Can't get external container port.")})?
            ;


        let client = ClientBuilder::rustls()
            .capabilities(caps)
            .connect(&format!("http://localhost:{}", container_port))
            .await
            .map_err(Error::WebDriverConnection)?;

        Ok(Browser {
            client,
            container_id,
            app_local_data_dir: self.app_local_data_dir,
            status: PhantomData,
        })
    }
}

impl Browser {
    /// Navigate to the given URL.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while executing `WebDriver` command.
    pub async fn goto(&mut self, url: &str) -> Result<()> {
        Ok(self.client.goto(url).await.map_err(Error::WebDriverCmd)?)
    }

    /// Get the current URL.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while executing `WebDriver` command.
    pub async fn current_url(&self) -> Result<String> {
        Ok(self
            .client
            .current_url()
            .await
            .map_err(Error::WebDriverCmd)?
            .to_string())
    }

    /// Get the HTML of the current page.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while executing `WebDriver` command.
    pub async fn html(&self) -> Result<String> {
        Ok(self
            .client
            .find(Locator::Css("html"))
            .await
            .map_err(Error::WebDriverCmd)?
            .html(false)
            .await
            .map_err(Error::WebDriverCmd)?)
    }

    /// Save a screenshot of the current page.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while executing `WebDriver` command or saving the screenshot.
    pub async fn save_screenshot(&self) -> Result<String> {
        let bytes = self
            .client
            .screenshot()
            .await
            .map_err(Error::WebDriverCmd)?;
        let file_path = format!("{}/screenshot.png", self.app_local_data_dir);
        std::fs::write(&file_path, bytes).map_err(Error::ScreenshotSave)?;

        Ok(file_path)
    }
}
