// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use crate::repo::models;
use crate::types::{DbPool, Result};
use tauri::State;
use tracing::instrument;

/// List models
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[tauri::command]
#[instrument(skip(pool))]
pub async fn list_models(pool: State<'_, DbPool>) -> Result<Vec<models::Model>> {
    models::list(&*pool).await
}
