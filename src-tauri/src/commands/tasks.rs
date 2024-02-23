// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// TODO(ri-nat): I don't really know, why Clippy is mad about these here, but let make him quiet for now.
#![allow(clippy::used_underscore_binding)]

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    repo::{
        self,
        tasks::{CreateParams, Status, Task},
        Pagination,
    },
    types::{DbPool, Result},
};

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
pub struct TasksList {
    pub tasks: Vec<Task>,
}

/// List all root tasks.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_root_tasks(pool: State<'_, DbPool>, pagination: Pagination) -> Result<TasksList> {
    let tasks = repo::tasks::list_roots(&*pool, pagination).await?;

    Ok(TasksList { tasks })
}

/// List child tasks by parent id.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_child_tasks(id: i64, pool: State<'_, DbPool>) -> Result<TasksList> {
    let task = repo::tasks::get(&*pool, id).await?;
    let tasks = repo::tasks::list_children(&*pool, id, task.ancestry.as_deref()).await?;

    Ok(TasksList { tasks })
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTask {
    pub agent_id: i64,
    pub title: String,
    pub summary: String,
    pub ancestry: Option<String>,
    pub status: Status,
}

/// Create new task.
///
/// # Errors
///
/// Returns error if there was a problem while inserting task into database.
#[tauri::command]
pub async fn create_task(request: CreateTask, pool: State<'_, DbPool>) -> Result<Task> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let task = repo::tasks::create(
        &mut *tx,
        CreateParams {
            agent_id: request.agent_id,
            origin_chat_id: None,
            title: &request.title,
            summary: &request.summary,
            status: request.status,
            ancestry: request.ancestry.as_deref(),
        },
    )
    .await?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(task)
}

/// Get task by id.
///
/// # Errors
///
/// Returns error if task with given id does not exist.
#[tauri::command]
pub async fn get_task(id: i64, pool: State<'_, DbPool>) -> Result<Task> {
    repo::tasks::get(&*pool, id).await
}

/// Delete task by id.
///
/// # Errors
///
/// Returns error if task with given id does not exist.
#[tauri::command]
pub async fn delete_task(id: i64, pool: State<'_, DbPool>) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let task = repo::tasks::get(&mut *tx, id).await?;

    repo::tasks::delete_children(&mut *tx, id, task.ancestry.as_deref()).await?;
    repo::tasks::delete(&mut *tx, id).await?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}
