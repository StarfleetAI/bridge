// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use crate::repo::task_results;
use crate::types::{DbPool, Result};
use tauri::State;
use tracing::instrument;

/// List task results by task id
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[tauri::command]
#[instrument(skip(pool))]
pub async fn list_task_results(
    pool: State<'_, DbPool>,
    task_id: i64,
) -> Result<Vec<task_results::TaskResult>> {
    task_results::list(&*pool, task_id).await
}

/// Get task text data by task result id
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[tauri::command]
#[instrument(skip(pool))]
pub async fn get_task_result_text_data(pool: State<'_, DbPool>, id: i64) -> Result<String> {
    task_results::get_text_data(&*pool, id).await
}
