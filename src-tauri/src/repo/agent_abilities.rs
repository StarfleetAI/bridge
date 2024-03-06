// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use sqlx::{query, query_as, query_scalar, Executor, Sqlite};

use crate::types::Result;

pub struct AgentAbility {
    pub agent_id: i64,
    pub ability_id: i64,
}

/// List all agent abilities.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn list<'a, E>(executor: E) -> Result<Vec<AgentAbility>>
where
    E: Executor<'a, Database = Sqlite>,
{
    Ok(query_as!(AgentAbility, "SELECT * FROM agent_abilities")
        .fetch_all(executor)
        .await
        .with_context(|| "Failed to list agent abilities")?)
}

/// Create agent ability.
///
/// # Errors
///
/// Returns error if there was a problem while creating agent ability.
pub async fn create<'a, E>(executor: E, agent_id: i64, ability_id: i64) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    query!(
        "INSERT INTO agent_abilities (agent_id, ability_id) VALUES ($1, $2)",
        agent_id,
        ability_id
    )
    .execute(executor)
    .await
    .with_context(|| "Failed to create agent ability")?;

    Ok(())
}

/// Delete agent ability.
///
/// # Errors
///
/// Returns error if there was a problem while deleting agent ability.
pub async fn delete_for_agent<'a, E>(executor: E, agent_id: i64) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    query!("DELETE FROM agent_abilities WHERE agent_id = $1", agent_id)
        .execute(executor)
        .await
        .with_context(|| "Failed to delete agent abilities for agent")?;

    Ok(())
}

/// Get agents count for ability.
///
/// # Errors
///
/// Returns error if there was a problem while fetching agents count for ability.
pub async fn get_agents_count<'a, E>(executor: E, ability_id: i64) -> Result<i32>
where
    E: Executor<'a, Database = Sqlite>,
{
    Ok(query_scalar!(
        "SELECT COUNT(*) as count FROM agent_abilities WHERE ability_id = $1",
        ability_id
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to get agents count for ability")?)
}
