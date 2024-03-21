// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{query, query_as, Executor, Sqlite};

use crate::types::Result;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Ability {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub code: String,
    pub parameters_json: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Ability {
    /// Constructs virtual ability for a function.
    ///
    /// # Panics
    ///
    /// Panics if `parameters_json` cannot be serialized.
    #[must_use]
    pub fn for_fn(description: &str, parameters_json: &Value) -> Self {
        Self {
            description: description.to_string(),
            parameters_json: serde_json::to_string(&parameters_json)
                .expect("failed to serialize parameters_json"),

            ..Default::default()
        }
    }
}

pub struct CreateParams {
    pub name: String,
    pub description: String,
    pub code: String,
    pub parameters_json: String,
}

pub struct UpdateParams {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub code: String,
    pub parameters_json: String,
}

/// List abilities for agent.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn list_for_agent<'a, E>(executor: E, agent_id: i64) -> Result<Vec<Ability>>
where
    E: Executor<'a, Database = Sqlite>,
{
    Ok(query_as!(
        Ability,
        r#"
        SELECT
            abilities.id as "id!", abilities.name, abilities.description, abilities.code,
            abilities.created_at, abilities.updated_at, abilities.parameters_json
        FROM abilities
        INNER JOIN agent_abilities ON abilities.id = agent_abilities.ability_id
        WHERE agent_abilities.agent_id = $1
        "#,
        agent_id
    )
    .fetch_all(executor)
    .await
    .with_context(|| "Failed to list abilities for agent")?)
}

/// List all abilities.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn list<'a, E>(executor: E) -> Result<Vec<Ability>>
where
    E: Executor<'a, Database = Sqlite>,
{
    Ok(query_as!(
        Ability,
        "SELECT id, name, description, code, parameters_json, created_at, updated_at FROM abilities ORDER BY id DESC"
    )
    .fetch_all(executor)
    .await
    .with_context(|| "Failed to list abilities")?)
}

/// Create ability.
///
/// # Errors
///
/// Returns error if there was a problem while creating ability.
pub async fn create<'a, E>(executor: E, params: CreateParams) -> Result<Ability>
where
    E: Executor<'a, Database = Sqlite>,
{
    let now = Utc::now();

    Ok(query_as!(
        Ability,
        r#"
        INSERT INTO abilities (name, description, code, parameters_json, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $5)
        RETURNING id, name, description, code, parameters_json, created_at, updated_at
        "#,
        params.name,
        params.description,
        params.code,
        params.parameters_json,
        now,
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to create ability")?)
}

/// Update ability.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn update<'a, E>(executor: E, params: UpdateParams) -> Result<Ability>
where
    E: Executor<'a, Database = Sqlite>,
{
    let now = Utc::now();

    Ok(query_as!(
        Ability,
        r#"
        UPDATE abilities
        SET name = $2, description = $3, code = $4, parameters_json = $5, updated_at = $6
        WHERE id = $1
        RETURNING id as "id!", name, description, code, parameters_json, created_at, updated_at
        "#,
        params.id,
        params.name,
        params.description,
        params.code,
        params.parameters_json,
        now
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to update ability")?)
}

/// Delete ability.
///
/// # Errors
///
/// Returns error if there was a problem while deleting ability.
pub async fn delete<'a, E>(executor: E, id: i64) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    query!("DELETE FROM abilities WHERE id = $1", id)
        .execute(executor)
        .await
        .with_context(|| "Failed to delete ability")?;

    Ok(())
}
