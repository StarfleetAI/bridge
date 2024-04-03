// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use tokio::fs;
use tracing::debug;

use crate::{repo::models::Provider, types::Result};

const DEFAULT_MODEL: &str = "OpenAI/gpt-3.5-turbo";
const SETTINGS_FILE: &str = "settings.json";
const DEFAULT_EXEC_STEPS_LIMIT: i64 = 12;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tasks {
    pub execution_concurrency: u16,
}

impl Default for Tasks {
    fn default() -> Self {
        Self {
            execution_concurrency: 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agents {
    #[serde(default = "default_execution_steps_limit")]
    pub execution_steps_limit: i64,
}
fn default_execution_steps_limit() -> i64 {
    DEFAULT_EXEC_STEPS_LIMIT
}

impl Default for Agents {
    fn default() -> Self {
        Self {
            execution_steps_limit: 12,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub default_model: Option<String>,
    #[serde(default)]
    pub api_keys: BTreeMap<Provider, String>,
    #[serde(default)]
    pub agents: Agents,
    #[serde(default)]
    pub tasks: Tasks,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to construct settings file path")]
    Path,
    #[error("failed to read settings file: {0}")]
    FileRead(std::io::Error),
    #[error("failed to write settings file: {0}")]
    FileWrite(std::io::Error),
    #[error("failed to parse settings file: {0}")]
    FileParse(serde_json::Error),
    #[error("failed to serialize settings: {0}")]
    JsonSerialization(serde_json::Error),
}

impl Settings {
    /// Load settings from disk. If the settings file doesn't exist, it will be created with default
    /// values.
    ///
    /// # Errors
    ///
    /// Will return an error if the settings file can't be read or if the settings can't be parsed.
    pub async fn load_from_disk(app_local_data_dir: &str) -> Result<Settings> {
        let path = Self::file_path(app_local_data_dir)?;

        if !Path::new(&path).exists() {
            debug!("Settings file not found, creating one");

            Self::default().save_to_disk(app_local_data_dir).await?;
        }

        let content = fs::read_to_string(&path).await.map_err(Error::FileRead)?;

        Ok(serde_json::from_str(&content).map_err(Error::FileParse)?)
    }

    /// Save settings to disk.
    ///
    /// # Errors
    ///
    /// Will return an error if the settings file can't be written.
    pub async fn save_to_disk(&self, app_local_data_dir: &str) -> Result<()> {
        fs::write(
            &Self::file_path(app_local_data_dir)?,
            serde_json::to_string_pretty(&self).map_err(Error::JsonSerialization)?,
        )
        .await
        .map_err(Error::FileWrite)?;

        Ok(())
    }

    /// Get the path to the settings file.
    ///
    /// # Errors
    ///
    /// Will return an error if the path can't be converted to a string.
    pub fn file_path(app_local_data_dir: &str) -> Result<String> {
        let mut path = PathBuf::new();
        path.push(app_local_data_dir);
        path.push(SETTINGS_FILE);

        path.into_os_string()
            .into_string()
            .map_err(|_| Error::Path.into())
    }

    /// Returns `default_model` if it's set, otherwise returns `DEFAULT_MODEL`.
    #[must_use]
    pub fn default_model(&self) -> &str {
        self.default_model.as_deref().unwrap_or(DEFAULT_MODEL)
    }
}
