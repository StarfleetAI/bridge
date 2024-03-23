// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use markdown::to_html;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::{Executor, Sqlite};

use crate::types::Result;

#[derive(
    Serialize, Deserialize, Debug, sqlx::Type, Default, PartialEq, Eq, Clone, Copy, Ord, PartialOrd,
)]
pub enum Kind {
    #[default]
    Text,
    Url,
}

impl From<String> for Kind {
    fn from(kind: String) -> Self {
        match kind.as_str() {
            "Url" => Kind::Url,
            _ => Kind::Text,
        }
    }
}

/// Safely render markdown in a reuslt data as an untrusted user input.
fn serialize_data<S>(data: &str, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&to_html(data))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskResult {
    pub id: i64,
    pub agent_id: i64,
    pub task_id: i64,
    pub kind: Kind,
    #[serde(serialize_with = "serialize_data")]
    pub data: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreateParams {
    pub agent_id: i64,
    pub task_id: i64,
    pub kind: Kind,
    pub data: String,
}

/// Create task result.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn create<'a, E>(executor: E, params: CreateParams) -> Result<TaskResult>
where
    E: Executor<'a, Database = Sqlite>,
{
    let now = Utc::now();

    Ok(sqlx::query_as!(
        TaskResult,
        r#"
        INSERT INTO task_results (agent_id, task_id, kind, data, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $5)
        RETURNING *
        "#,
        params.agent_id,
        params.task_id,
        params.kind,
        params.data,
        now,
    )
    .fetch_one(executor)
    .await?)
}

/// Get task results by task id
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn get<'a, E>(executor: E, task_id: i64) -> Result<Vec<TaskResult>>
where
    E: Executor<'a, Database = Sqlite>,
{
    let task_results = sqlx::query_as!(
        TaskResult,
        r#"
        SELECT 
            id as "id!", agent_id, task_id, kind, data, created_at, updated_at
        FROM task_results WHERE task_id = $1
        ORDER BY id ASC
        "#,
        task_id
    )
    .fetch_all(executor)
    .await
    .with_context(|| "Failed to fetch task results")?;
    Ok(task_results)
}
