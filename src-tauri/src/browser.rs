// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use askama::Template;
use std::marker::PhantomData;
use std::time::Duration;

use fantoccini::{wd::Capabilities, Client, ClientBuilder, Locator};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::runtime::Handle;
use tokio::task;
use tokio::time::sleep;
use tracing::{debug, error};

use crate::docker::ContainerManager;
use crate::types::Result;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to connect to WebDriver: {0}")]
    WebDriverConnection(#[from] fantoccini::error::NewSessionError),
    #[error("failed to execute WebDriver command: {0}")]
    WebDriverCmd(#[from] fantoccini::error::CmdError),
    #[error("failed to get WebDriver host port binding")]
    WebDriverHostPort,
    #[error("failed to save screenshot: {0}")]
    ScreenshotSave(#[from] std::io::Error),
}

/// Stores virtual browser data.
#[derive(Debug)]
pub struct Browser {
    /// Folder where the screenshots and downloaded files will be stored.
    pub workdir: String,
    /// WebDriver Client instance.
    pub client: Client,
    /// Chromedriver container identifier.
    pub container_id: String,
    /// Browser status.
    status: PhantomData<()>,
}

/// Constructs browser instances.
///
/// This type itself is not particularly useful. It only creates browser instances.
#[allow(clippy::module_name_repetitions)]
pub struct BrowserBuilder {
    /// Folder where the screenshots and downloaded files will be stored.
    workdir: String,
}

#[derive(Template)]
#[template(path = "js/list_viewport_elements.js", escape = "none")]
struct ListViewportElementsTemplate {}

#[derive(Debug, Serialize, Deserialize)]
pub enum ElementType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "button")]
    Button,
    #[serde(rename = "input")]
    Input,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: ElementType,
    pub content: Option<String>,
}

impl BrowserBuilder {
    /// Create a new instance of itself.
    #[must_use]
    pub fn new(workdir: &str) -> Self {
        Self {
            workdir: workdir.to_string(),
        }
    }

    /// The Browser instance initialisation.
    ///
    /// Creates the personal chromedriver container, connects to it, saves the necessary data into Browser attributes.
    /// # Errors
    ///
    /// Returns error if there was a problem while connecting to `WebDriver`.
    pub async fn connect(self) -> Result<Browser> {
        let mut caps = Capabilities::new();
        // TODO: support geckodriver
        let opts = json!({
            "args": ["--headless", "--disable-gpu", "--no-sandbox", "--disable-dev-shm-usage"],
        });
        caps.insert("goog:chromeOptions".to_string(), opts);

        let docker_client = ContainerManager::get().await?;
        let container_id = docker_client.launch_chromedriver_container().await?;

        let host_port = Self::wait_for_host_port(docker_client, &container_id).await?;

        let client = ClientBuilder::rustls()
            .capabilities(caps)
            .connect(&format!("http://localhost:{host_port}"))
            .await
            .map_err(Error::WebDriverConnection)?;

        client
            .set_window_size(1920, 1080)
            .await
            .map_err(Error::WebDriverCmd)?;

        Ok(Browser {
            client,
            container_id,
            workdir: self.workdir,
            status: PhantomData,
        })
    }

    async fn wait_for_host_port(
        docker_client: &ContainerManager,
        container_id: &str,
    ) -> Result<String> {
        for _ in 0..30 {
            let container_info = docker_client.inspect_container(container_id).await?;

            if let Some(port) = container_info
                .network_settings
                .as_ref()
                .and_then(|network_settings| network_settings.ports.as_ref())
                .and_then(|ports| ports.get("9515/tcp"))
                .and_then(|maybe_port_bindings| maybe_port_bindings.as_ref())
                .and_then(|port_bindings| port_bindings.first())
                .and_then(|port_binding| port_binding.host_port.as_deref())
            {
                return Ok(port.to_string());
            }

            debug!("Port 9515 is not bound yet, waiting...");

            sleep(Duration::from_millis(500)).await;
        }

        Err(Error::WebDriverHostPort.into())
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
    pub async fn get_current_url(&self) -> Result<String> {
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
    pub async fn get_html(&self) -> Result<String> {
        Ok(self
            .find(Locator::Css("html"))
            .await?
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

        let file_path = format!("{}/screenshot.png", self.workdir);
        std::fs::write(&file_path, bytes).map_err(Error::ScreenshotSave)?;

        Ok(file_path)
    }

    /// Get meaningful elements from the current viewport.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while executing `WebDriver` command.
    pub async fn list_viewport_elements(&self) -> Result<Vec<Element>> {
        let script_template = ListViewportElementsTemplate {};
        let content = script_template
            .render()
            .with_context(|| "Failed to render `call_tools` script")?;

        let result = self
            .client
            .execute(&content, vec![])
            .await
            .map_err(Error::WebDriverCmd)?;
        debug!("Elements from viewport: {result}");

        Ok(serde_json::from_value(result.clone())
            .with_context(|| format!("Failed to parse elements from result: {result}"))?)
    }

    /// Scrolls one screen down.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while executing `WebDriver` command.
    pub async fn scroll_down(&self) -> Result<()> {
        self.client
            .execute("window.scrollBy(0, window.innerHeight)", vec![])
            .await
            .map_err(Error::WebDriverCmd)?;

        Ok(())
    }

    /// Scrolls one screen up.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while executing `WebDriver` command.
    pub async fn scroll_up(&self) -> Result<()> {
        self.client
            .execute("window.scrollBy(0, -window.innerHeight)", vec![])
            .await
            .map_err(Error::WebDriverCmd)?;

        Ok(())
    }

    /// Calculates scroll position percentage.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while executing `WebDriver` command.
    pub async fn get_scroll_position(&self) -> Result<i64> {
        let scroll_top = self
            .client
            .execute("return window.scrollY", vec![])
            .await
            .map_err(Error::WebDriverCmd)?;
        let scroll_height = self
            .client
            .execute("return document.body.scrollHeight", vec![])
            .await
            .map_err(Error::WebDriverCmd)?;
        let client_height = self
            .client
            .execute("return window.innerHeight", vec![])
            .await
            .map_err(Error::WebDriverCmd)?;
        let scroll_position = scroll_top.as_f64().unwrap_or_default()
            / (scroll_height.as_f64().unwrap_or_default()
                - client_height.as_f64().unwrap_or_default());

        #[allow(clippy::cast_possible_truncation)]
        Ok((scroll_position * 100.0).ceil() as i64)
    }

    /// Clicks on the element with a given `data-sfai` attribute value.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while executing `WebDriver` command.
    pub async fn click(&self, id: i64) -> Result<()> {
        self.client
            .execute(
                &format!("document.querySelector('[data-sfai=\"{id}\"]').click()"),
                vec![],
            )
            .await
            .map_err(Error::WebDriverCmd)?;

        Ok(())
    }

    /// Sends keys to the element with a given `data-sfai` attribute value.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while executing `WebDriver` command.
    pub async fn send_keys(&self, id: i64, text: &str) -> Result<()> {
        self.find(Locator::Css(&format!("[data-sfai=\"{id}\"]")))
            .await?
            .send_keys(text)
            .await
            .map_err(Error::WebDriverCmd)?;

        Ok(())
    }

    async fn find(&self, locator: Locator<'_>) -> Result<fantoccini::elements::Element> {
        Ok(self
            .client
            .find(locator)
            .await
            .map_err(Error::WebDriverCmd)?)
    }
}

impl Drop for Browser {
    fn drop(&mut self) {
        let container_id = self.container_id.clone();

        task::block_in_place(move || {
            Handle::current().block_on(async move {
                let docker_client = match ContainerManager::get().await {
                    Ok(client) => client,
                    Err(e) => {
                        error!("Can't get container manager to kill container: {e}");
                        return;
                    }
                };

                if let Err(e) = docker_client.kill_container(&container_id).await {
                    error!("Can't kill container {container_id}: {e}");
                }
            });
        });
    }
}
