// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Executor, Sqlite};
use std::path::PathBuf;
use tauri::AppHandle;
use tokio::fs::create_dir_all;

use crate::types::Result;

use super::Pagination;

#[derive(Serialize, Deserialize, Debug, sqlx::Type, PartialEq, Default, Clone, Copy)]
pub enum Status {
    /// Task is in draft and has not been selected for execution yet.
    #[default]
    Draft,
    /// Task is selected for execution.
    ToDo,
    /// Task is currently being executed.
    InProgress,
    /// Task is waiting for a user input.
    WaitingForUser,
    /// Task is paused by the user.
    Paused,
    /// Task is completed.
    Done,
    /// Task execution failed.
    Failed,
    /// Task canceled by the user.
    Canceled,
}

impl From<String> for Status {
    fn from(status: String) -> Self {
        match status.as_str() {
            "ToDo" => Status::ToDo,
            "InProgress" => Status::InProgress,
            "WaitingForUser" => Status::WaitingForUser,
            "Paused" => Status::Paused,
            "Done" => Status::Done,
            "Failed" => Status::Failed,
            "Canceled" => Status::Canceled,
            _ => Status::Draft,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Task {
    pub id: i64,
    pub agent_id: i64,
    /// Chat from which this task was created.
    pub origin_chat_id: Option<i64>,
    /// Chat from which this task is being controlled (between the user and the Bridge).
    pub control_chat_id: Option<i64>,
    /// Chat in which this task is being executed (between the Bridge and the agent).
    pub execution_chat_id: Option<i64>,
    pub title: String,
    pub summary: String,
    pub status: Status,
    /// Task's parent ids in a form of `1/2/3`. `None` for root tasks.
    pub ancestry: Option<String>,
    pub ancestry_level: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Task {
    /// Returns parent id of the task.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while parsing parent id.
    pub fn parent_id(&self) -> Result<Option<i64>> {
        Ok(match self.ancestry {
            Some(ref ancestry) => {
                let segment = ancestry
                    .split('/')
                    .last()
                    .context("No segments found in ancestry")?;

                Some(
                    segment.parse::<i64>().with_context(|| {
                        "Failed to parse parent id from ancestry segment {segment}"
                    })?,
                )
            }
            None => None,
        })
    }

    #[must_use]
    pub fn children_ancestry(&self) -> String {
        match self.ancestry {
            Some(ref ancestry) => format!("{}/{}", ancestry, self.id),
            None => self.id.to_string(),
        }
    }

    /// Returns workdir for the task.
    ///
    /// # Errors
    ///
    /// Returns error if there was a problem while building workdir path.
    pub async fn workdir(&self, app_handle: &AppHandle) -> Result<PathBuf> {
        let workdir_name = format!(
            "wd-task-{}",
            self.workdir_id().context("Failed to get workdir ID")?
        );

        // Build workdir path
        let mut workdir = PathBuf::new();
        workdir.push(
            app_handle
                .path_resolver()
                .app_local_data_dir()
                .context("Failed to get app local data dir")?,
        );
        workdir.push(workdir_name);

        if !workdir.exists() {
            create_dir_all(&workdir)
                .await
                .with_context(|| "Failed to create workdir")?;
        }

        Ok(workdir)
    }

    fn workdir_id(&self) -> Result<i64> {
        Ok(match self.ancestry {
            Some(ref ancestry) => ancestry
                .split('/')
                .collect::<Vec<_>>()
                .first()
                .context("No segments found in ancestry")?
                .parse::<i64>()
                .context("Failed to parse workdir id")?,
            None => self.id,
        })
    }
}

#[derive(Debug, Default)]
pub struct CreateParams<'a> {
    pub agent_id: i64,
    /// Chat from which this task was created.
    pub origin_chat_id: Option<i64>,
    pub title: &'a str,
    pub summary: Option<&'a str>,
    pub status: Status,
    /// Task's parent ids in a form of `1/2/3`. `None` for root tasks.
    pub ancestry: Option<&'a str>,
}

pub struct UpdateParams<'a> {
    pub id: i64,
    pub title: &'a str,
    pub summary: &'a str,
    pub agent_id: i64,
}

/// Gets root task for execution.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn get_root_for_execution<'a, E: Executor<'a, Database = Sqlite>>(
    executor: E,
) -> Result<Option<Task>> {
    Ok(query_as!(
        Task,
        r#"
        SELECT
            id as "id!",
            agent_id,
            origin_chat_id,
            control_chat_id,
            execution_chat_id,
            title,
            summary,
            status,
            ancestry,
            ancestry_level,
            created_at,
            updated_at
        FROM tasks
        WHERE ancestry IS NULL
        AND status = $1
        ORDER BY created_at ASC
        LIMIT 1
        "#,
        Status::ToDo,
    )
    .fetch_optional(executor)
    .await
    .context("Failed to list tasks")?)
}

/// Gets child task for execution.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn get_children_for_execution<'a, E: Executor<'a, Database = Sqlite>>(
    executor: E,
    ancestry: &'a str,
) -> Result<Option<Task>> {
    Ok(query_as!(
        Task,
        r#"
        SELECT
            id as "id!",
            agent_id,
            origin_chat_id,
            control_chat_id,
            execution_chat_id,
            title,
            summary,
            status,
            ancestry,
            ancestry_level,
            created_at,
            updated_at
        FROM tasks
        WHERE ancestry = $1
        AND status != $2
        ORDER BY created_at ASC
        LIMIT 1
        "#,
        ancestry,
        Status::Done,
    )
    .fetch_optional(executor)
    .await
    .context("Failed to list tasks")?)
}

/// List all tasks.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn list_roots<'a, E: Executor<'a, Database = Sqlite>>(
    executor: E,
    pagination: Pagination,
) -> Result<Vec<Task>> {
    if pagination.page < 1 {
        return Err(anyhow::anyhow!("`page` number must be greater than 0").into());
    }

    if pagination.per_page < 1 {
        return Err(anyhow::anyhow!("`per_page` number must be greater than 0").into());
    }

    let offset = (pagination.page - 1) * pagination.per_page;

    Ok(query_as!(
        Task,
        r#"
        SELECT
            id as "id!",
            agent_id,
            origin_chat_id,
            control_chat_id,
            execution_chat_id,
            title,
            summary,
            status,
            ancestry,
            ancestry_level,
            created_at,
            updated_at
        FROM tasks
        WHERE ancestry IS NULL
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        pagination.per_page,
        offset,
    )
    .fetch_all(executor)
    .await
    .context("Failed to list tasks")?)
}

/// List all child tasks for given task.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn list_children<'a, E: Executor<'a, Database = Sqlite>>(
    executor: E,
    id: i64,
    ancestry: Option<&'a str>,
) -> Result<Vec<Task>> {
    let current_ancestry_level: i64 = match ancestry {
        Some(ancestry) => {
            let count = ancestry.split('/').count();

            match count.try_into() {
                Ok(ancestry_level) => ancestry_level,
                Err(_) => return Err(anyhow::anyhow!("Too many ancestors").into()),
            }
        }
        None => 0,
    };

    let children_ancestry_level = current_ancestry_level
        .checked_add(1)
        .ok_or_else(|| anyhow::anyhow!("Maximum ancestry level reached for task with id: {id}"))?;

    let children_ancestry = if let Some(ancestry) = ancestry {
        format!("{ancestry}/{id}/%")
    } else {
        format!("{id}/%")
    };

    Ok(query_as!(
        Task,
        r#"
        SELECT
            id as "id!",
            agent_id,
            origin_chat_id,
            control_chat_id,
            execution_chat_id,
            title,
            summary,
            status,
            ancestry,
            ancestry_level,
            created_at,
            updated_at
        FROM tasks
        WHERE ancestry LIKE $1
        AND ancestry_level = $2
        ORDER BY created_at DESC
        "#,
        children_ancestry,
        children_ancestry_level,
    )
    .fetch_all(executor)
    .await
    .context("Failed to list tasks")?)
}

/// Create new task.
///
/// # Errors
///
/// Returns error if there was a problem while inserting new task.
pub async fn create<'a, E: Executor<'a, Database = Sqlite>>(
    executor: E,
    params: CreateParams<'a>,
) -> Result<Task> {
    let now = Utc::now().naive_utc();

    let ancestry_level = match params.ancestry {
        Some(ancestry) => {
            let count = ancestry.split('/').count();

            match count.try_into() {
                Ok(ancestry_level) => ancestry_level,
                Err(_) => return Err(anyhow::anyhow!("Too many ancestors").into()),
            }
        }
        None => 0,
    };

    let task = query_as!(
        Task,
        r#"
        INSERT INTO tasks (
            agent_id,
            origin_chat_id,
            title,
            summary,
            status,
            ancestry,
            ancestry_level,
            created_at,
            updated_at
        )
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $8
        )
        RETURNING
            id as "id!",
            agent_id,
            origin_chat_id,
            control_chat_id,
            execution_chat_id,
            title,
            summary,
            status,
            ancestry,
            ancestry_level,
            created_at,
            updated_at
        "#,
        params.agent_id,
        params.origin_chat_id,
        params.title,
        params.summary,
        params.status,
        params.ancestry,
        ancestry_level,
        now,
    )
    .fetch_one(executor)
    .await
    .context("Failed to create task")?;

    Ok(task)
}

/// Update task title or/and summary by id
///
/// # Errors
///
/// Returns error if there was a problem while updating task.
pub async fn update<'a, E: Executor<'a, Database = Sqlite>>(
    executor: E,
    params: UpdateParams<'a>,
) -> Result<Task> {
    let now = Utc::now().naive_utc();

    let task = query_as!(
        Task,
        r#"
        UPDATE tasks
        SET
            title = COALESCE($1, title),
            summary = COALESCE($2, summary),
            updated_at = $3,
            agent_id = $4
        WHERE id = $5
        RETURNING
            id as "id!",
            agent_id,
            origin_chat_id,
            control_chat_id,
            execution_chat_id,
            title,
            summary,
            status,
            ancestry,
            ancestry_level,
            created_at,
            updated_at
        "#,
        params.title,
        params.summary,
        now,
        params.agent_id,
        params.id,
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to update task")?;
    Ok(task)
}

/// Update task status by id.
///
/// # Errors
///
/// Returns error if there was a problem while updating task status.
pub async fn update_status<'a, E: Executor<'a, Database = Sqlite>>(
    executor: E,
    id: i64,
    status: Status,
) -> Result<Task> {
    let now = Utc::now().naive_utc();
    let task = query_as!(
        Task,
        r#"
        UPDATE tasks
        SET
            status = $1,
            updated_at = $2
        WHERE id = $3
        RETURNING
            id as "id!",
            agent_id,
            origin_chat_id,
            control_chat_id,
            execution_chat_id,
            title,
            summary,
            status,
            ancestry,
            ancestry_level,
            created_at,
            updated_at
        "#,
        status,
        now,
        id,
    )
    .fetch_one(executor)
    .await
    .context("Failed to update task status")?;

    Ok(task)
}

/// Update task execution chat id by id.
///
/// # Errors
///
/// Returns error if there was a problem while updating task execution chat id.
pub async fn update_execution_chat_id<'a, E: Executor<'a, Database = Sqlite>>(
    executor: E,
    id: i64,
    execution_chat_id: i64,
) -> Result<()> {
    let now = Utc::now().naive_utc();
    query!(
        r#"
        UPDATE tasks
        SET
            execution_chat_id = $1,
            updated_at = $2
        WHERE id = $3
        "#,
        execution_chat_id,
        now,
        id,
    )
    .execute(executor)
    .await
    .context("Failed to update task execution chat id")?;

    Ok(())
}

/// Revise task by id.
///
/// # Errors
///
/// Returns error if there was a problem while revising task.
pub async fn revise<'a, E: Executor<'a, Database = Sqlite>>(executor: E, id: i64) -> Result<Task> {
    update_status(executor, id, Status::Draft).await
}

/// Cancel task by id.
///
/// # Errors
///
/// Returns error if there was a problem while canceling task.
pub async fn cancel<'a, E: Executor<'a, Database = Sqlite>>(executor: E, id: i64) -> Result<Task> {
    update_status(executor, id, Status::Canceled).await
}

/// Pause task by id.
///
/// # Errors
///
/// Returns error if there was a problem while pausing task.
pub async fn pause<'a, E: Executor<'a, Database = Sqlite>>(executor: E, id: i64) -> Result<Task> {
    update_status(executor, id, Status::Paused).await
}

/// Execute task by id.
///
/// # Errors
///
/// Returns error if there was a problem while executing task.
pub async fn execute<'a, E: Executor<'a, Database = Sqlite>>(executor: E, id: i64) -> Result<Task> {
    update_status(executor, id, Status::ToDo).await
}

/// Start task by id.
///
/// # Errors
///
/// Returns error if there was a problem while starting task.
pub async fn start_progress<'a, E: Executor<'a, Database = Sqlite>>(
    executor: E,
    id: i64,
) -> Result<Task> {
    update_status(executor, id, Status::InProgress).await
}

/// Marks task as waiting for user input by id.
///
/// # Errors
///
/// Returns error if there was a problem while marking task as waiting for user input.
pub async fn wait_for_user<'a, E: Executor<'a, Database = Sqlite>>(
    executor: E,
    id: i64,
) -> Result<Task> {
    update_status(executor, id, Status::WaitingForUser).await
}

/// Fail task by id.
///
/// # Errors
///
/// Returns error if there was a problem while failing task.
pub async fn fail<'a, E: Executor<'a, Database = Sqlite>>(executor: E, id: i64) -> Result<Task> {
    update_status(executor, id, Status::Failed).await
}

/// Complete task by id.
///
/// # Errors
///
/// Returns error if there was a problem while completing task.
pub async fn complete<'a, E: Executor<'a, Database = Sqlite>>(
    executor: E,
    id: i64,
) -> Result<Task> {
    update_status(executor, id, Status::Done).await
}

/// Get task by id.
///
/// # Errors
///
/// Returns error if there was a problem while fetching task.
pub async fn get<'a, E: Executor<'a, Database = Sqlite>>(executor: E, id: i64) -> Result<Task> {
    let task = query_as!(
        Task,
        r#"
        SELECT
            id as "id!",
            agent_id,
            origin_chat_id,
            control_chat_id,
            execution_chat_id,
            title,
            summary,
            status,
            ancestry,
            ancestry_level,
            created_at,
            updated_at
        FROM tasks
        WHERE id = $1
        "#,
        id,
    )
    .fetch_one(executor)
    .await
    .context("Failed to get task")?;

    Ok(task)
}

/// Delete task by id.
///
/// # Errors
///
/// Returns error if there was a problem while deleting task.
pub async fn delete<'a, E: Executor<'a, Database = Sqlite>>(executor: E, id: i64) -> Result<()> {
    query!("DELETE FROM tasks WHERE id = $1", id)
        .execute(executor)
        .await
        .context("Failed to delete task")?;

    Ok(())
}

/// Delete child tasks by parent id and ancestry.
///
/// # Errors
///
/// Returns error if there was a problem while deleting tasks.
pub async fn delete_children<'a, E: Executor<'a, Database = Sqlite>>(
    executor: E,
    id: i64,
    ancestry: Option<&'a str>,
) -> Result<()> {
    let children_ancestry = if let Some(ancestry) = ancestry {
        format!("{ancestry}/{id}/%")
    } else {
        format!("{id}/%")
    };

    query!(
        "DELETE FROM tasks WHERE ancestry LIKE $1",
        children_ancestry
    )
    .execute(executor)
    .await
    .context("Failed to delete tasks")?;

    Ok(())
}

/// Delete tasks from chat.
///
/// # Errors
///
/// Returns error if there was a problem while deleting `tasks` records.
pub async fn delete_for_chat<'a, E: Executor<'a, Database = Sqlite>>(
    executor: E,
    chat_id: i64,
) -> Result<()> {
    query!(
        "DELETE FROM tasks WHERE origin_chat_id = $1 OR control_chat_id = $1 OR execution_chat_id = $1",
        chat_id
    )
        .execute(executor)
        .await
        .context("Failed to delete `tasks` records")?;

    Ok(())
}

/// Transitions tasks from one status to another.
///
/// # Errors
///
/// Returns error if there was a problem while updating messages.
pub async fn transition_all<'a, E>(executor: E, from: Status, to: Status) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    query!("UPDATE tasks SET status = $1 WHERE status = $2", to, from)
        .execute(executor)
        .await
        .with_context(|| "Failed to set `{from}` tasks to `{to}`")?;

    Ok(())
}

/// Assigns tasks to agent by id.
///
/// # Errors
///
/// Returns error if there was a problem while assigning tasks to agent.
pub async fn assign<'a, E: Executor<'a, Database = Sqlite>>(
    executor: E,
    task_id: i64,
    agent_id: i64,
) -> Result<()> {
    query!(
        "UPDATE tasks SET agent_id = $1 WHERE id = $2",
        agent_id,
        task_id
    )
    .execute(executor)
    .await
    .context("Failed to assign task to agent")?;

    Ok(())
}
