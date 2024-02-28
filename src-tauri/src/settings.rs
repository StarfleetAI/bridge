// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::types::Result;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    pub openai_api_key: Option<String>,
    pub python_path: Option<String>,
    pub agents: Value,
}

impl Settings {
    #[must_use]
    pub fn new() -> Self {
        Self {
            openai_api_key: None,
            python_path: None,
            agents: json!({}),
        }
    }

    /// Load settings from disk. If the settings file doesn't exist, it will be created with default
    /// values.
    ///
    /// # Errors
    ///
    /// Will return an error if the settings file can't be read or parsed.
    ///
    /// # Panics
    ///
    /// Will panic if the settings file path can't be converted to a string.
    pub async fn load_from_disk(app_local_data_dir: &str) -> Result<Settings> {
        let mut path = PathBuf::new();
        path.push(app_local_data_dir);
        path.push("settings.json");

        let settings_path = path
            .into_os_string()
            .into_string()
            .expect("Failed to convert settings.json path to string");

        debug!("Settings path: {}", settings_path);

        if !Path::new(&settings_path).exists() {
            debug!("Settings file not found, creating one");
            let settings = Settings::new();

            fs::write(
                &settings_path,
                serde_json::to_string_pretty(&settings).unwrap(),
            )
            .await
            .with_context(|| format!("Failed to write settings to {settings_path}"))?;
        }

        let settings = fs::read_to_string(&settings_path)
            .await
            .with_context(|| format!("Failed to read settings from {settings_path}"))?;

        Ok(serde_json::from_str(&settings).with_context(|| "Failed to parse settings")?)
    }
}
