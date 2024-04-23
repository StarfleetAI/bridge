// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use bridge_common::{repo::models, types::models::Model};
use tauri::State;
use tracing::instrument;

use crate::types::{DbPool, Result};

/// List models
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[tauri::command]
#[instrument(skip(pool))]
pub async fn list_models(pool: State<'_, DbPool>) -> Result<Vec<Model>> {
    Ok(models::list(&*pool, crate::CID).await?)
}
