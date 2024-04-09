// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

use crate::task_planner::TaskPlanner;
use crate::{
    repo::{
        self,
        tasks::{CreateParams, Status, Task, UpdateParams},
        Pagination,
    },
    types::{DbPool, Result},
};

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
pub struct TasksList {
    pub tasks: Vec<Task>,
    pub count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTask {
    pub agent_id: i64,
    pub title: String,
    pub summary: Option<String>,
    pub ancestry: Option<String>,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTask {
    pub id: i64,
    pub title: String,
    pub summary: String,
    pub agent_id: i64,
}

/// Plan task by id.
///
/// # Errors
///
/// Returns error if task with given id does not exist or there was a problem while planning task.
#[tauri::command]
pub async fn plan_task(app_handle: AppHandle, pool: State<'_, DbPool>, id: i64) -> Result<()> {
    let mut task = repo::tasks::get(&*pool, id).await?;

    TaskPlanner::new(&app_handle).plan(&mut task).await
}

/// Create new task.
///
/// # Errors
///
/// Returns error if there was a problem while inserting task into database.
#[tauri::command]
pub async fn create_task(request: CreateTask, pool: State<'_, DbPool>) -> Result<Task> {
    repo::tasks::create(
        &*pool,
        CreateParams {
            agent_id: request.agent_id,
            origin_chat_id: None,
            title: &request.title,
            summary: request.summary.as_deref(),
            status: request.status,
            ancestry: request.ancestry.as_deref(),
        },
    )
    .await
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

    // TODO: delete `execution` and `control` chats; `messages` (for both of them); `task_results`.
    //       As well as the working directory (for the root task).
    //       Also, delete all of these for the children tasks.

    repo::tasks::delete_children(&mut *tx, id, task.ancestry.as_deref()).await?;
    repo::tasks::delete(&mut *tx, id).await?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}

/// Execute task by id.
///
/// # Errors
///
/// Returns error if task with given id does not exist.
#[tauri::command]
pub async fn execute_task(id: i64, pool: State<'_, DbPool>) -> Result<Task> {
    let task = repo::tasks::get(&*pool, id).await?;

    // Delete all the task progress and the results if task is being re-executed
    if task.status == Status::Done {
        repo::task_results::delete_for_task(&*pool, id).await?;
        repo::messages::delete_for_chat(
            &*pool,
            task.execution_chat_id
                .context("No execution chat ID for task")?,
        )
        .await?;
    }

    repo::tasks::execute(&*pool, id).await
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

/// List child tasks by parent id.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_child_tasks(id: i64, pool: State<'_, DbPool>) -> Result<TasksList> {
    let task = repo::tasks::get(&*pool, id).await?;
    let tasks = repo::tasks::list_direct_children(&*pool, &task).await?;

    Ok(TasksList { tasks, count: None })
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

    Ok(TasksList { tasks, count: None })
}

/// List root tasks by status.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_root_tasks_by_status(
    status: Status,
    pool: State<'_, DbPool>,
    pagination: Pagination,
) -> Result<TasksList> {
    let tasks = repo::tasks::list_roots_by_status(&*pool, status, pagination).await?;
    let count = repo::tasks::get_total_number_by_status(&*pool, status).await?;

    Ok(TasksList {
        tasks,
        count: Some(count),
    })
}

/// Revise task by id.
///
/// # Errors
///
/// Returns error if task with given id does not exist.
#[tauri::command]
pub async fn revise_task(id: i64, pool: State<'_, DbPool>) -> Result<Task> {
    repo::tasks::revise(&*pool, id).await
}

/// Update task title or/and summary by id. Title and summary can be optional
///
/// # Errors
///
/// Returns error if task with given id does not exist
#[tauri::command]
pub async fn update_task(request: UpdateTask, pool: State<'_, DbPool>) -> Result<Task> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    if request.title.is_empty() && request.summary.is_empty() {
        return Err(anyhow!("Title and summary cannot be both empty").into());
    }

    let task = repo::tasks::update(
        &mut *tx,
        UpdateParams {
            id: request.id,
            title: &request.title,
            summary: &request.summary,
            agent_id: request.agent_id,
        },
    )
    .await?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(task)
}

/// Duplicate task by id.
///
/// # Errors
///
/// Returns error if task with given id does not exist.
#[tauri::command]
pub async fn duplicate_task(id: i64, pool: State<'_, DbPool>) -> Result<Task> {
    let task = repo::tasks::get(&*pool, id).await?;
    repo::tasks::create(
        &*pool,
        CreateParams {
            status: Status::Draft,
            agent_id: task.agent_id,
            origin_chat_id: task.origin_chat_id,
            title: &task.title,
            summary: Some(&task.summary),
            ancestry: task.ancestry.as_deref(),
        },
    )
    .await
}
