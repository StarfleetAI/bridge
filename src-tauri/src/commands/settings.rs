// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use bridge_common::{repo, settings::Settings};
use tauri::State;
use tokio::sync::RwLock;

use crate::types::{DbPool, Result};

/// Get the current settings.
///
/// # Errors
///
/// Does not return an error.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn get_settings(settings: State<'_, RwLock<Settings>>) -> Result<Settings> {
    Ok(settings.read().await.clone())
}

/// Update the settings.
///
/// # Errors
///
/// Will return an error if the settings can't be saved to disk.
///
/// # Panics
///
/// Will panic if the app local data dir can't be resolved.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn update_settings(
    settings: State<'_, RwLock<Settings>>,
    pool: State<'_, DbPool>,
    new_settings: Settings,
) -> Result<()> {
    let mut st = settings.write().await;
    *st = new_settings;

    repo::settings::update(&*pool, crate::CID, &st).await?;

    Ok(())
}
