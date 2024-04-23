// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use crate::types::{DbPool, Result};
use bridge_common::{repo, types::task_results::TaskResult};
use tauri::State;
use tracing::instrument;

/// List task results by task id
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[tauri::command]
#[instrument(skip(pool))]
pub async fn list_task_results(pool: State<'_, DbPool>, task_id: i32) -> Result<Vec<TaskResult>> {
    Ok(repo::task_results::list(&*pool, crate::CID, task_id).await?)
}

/// Get task text data by task result id
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[tauri::command]
#[instrument(skip(pool))]
pub async fn get_task_result_text_data(pool: State<'_, DbPool>, id: i32) -> Result<String> {
    Ok(repo::task_results::get_text_data(&*pool, crate::CID, id).await?)
}
