// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// TODO(ri-nat): I don't really know, why Clippy is mad about these here, but let make him quiet for now.
#![allow(clippy::used_underscore_binding)]

use std::collections::BTreeMap;

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use tauri::State;

use crate::types::{DbMutex, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Agent {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub system_message: String,
    pub ability_ids: Vec<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AgentsList {
    pub agents: Vec<Agent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAgent {
    pub name: String,
    pub description: String,
    pub system_message: String,
    pub ability_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAgent {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub system_message: String,
    pub ability_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteAgent {
    pub id: i64,
}

pub struct AgentRow {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub system_message: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

struct AgentAbilityRow {
    agent_id: i64,
    ability_id: i64,
}

/// List all agents.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_agents(pool_mutex: State<'_, DbMutex>) -> Result<AgentsList> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let rows = query_as!(
        AgentRow,
        r#"
        SELECT id, name, description, system_message, created_at, updated_at
        FROM agents
        ORDER BY id DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .with_context(|| "Failed to fetch agents")?;

    let ability_rows = query_as!(
        AgentAbilityRow,
        "SELECT agent_id, ability_id FROM agent_abilities",
    )
    .fetch_all(pool)
    .await
    .with_context(|| "Failed to fetch agent abilities")?;

    let mut abilities: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
    for row in ability_rows {
        abilities
            .entry(row.agent_id)
            .or_default()
            .push(row.ability_id);
    }

    let agents = rows
        .into_iter()
        .map(|row| Agent {
            id: row.id,
            name: row.name,
            description: row.description,
            system_message: row.system_message,
            ability_ids: abilities.get(&row.id).unwrap_or(&Vec::new()).clone(),
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
        .collect();

    Ok(AgentsList { agents })
}

/// Create new agent.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[tauri::command]
pub async fn create_agent(request: CreateAgent, pool_mutex: State<'_, DbMutex>) -> Result<Agent> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let now = Utc::now();
    let agent = query_as!(
        AgentRow,
        r#"
        INSERT INTO agents (name, description, system_message, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        RETURNING id, name, description, system_message, created_at, updated_at
        "#,
        request.name,
        request.description,
        request.system_message,
        now,
    )
    .fetch_one(&mut *tx)
    .await
    .with_context(|| "Failed to insert agent")?;

    for ability_id in &request.ability_ids {
        query!(
            "INSERT INTO agent_abilities (agent_id, ability_id) VALUES ($1, $2)",
            agent.id,
            ability_id,
        )
        .execute(&mut *tx)
        .await
        .with_context(|| "Failed to insert agent ability")?;
    }

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(Agent {
        id: agent.id,
        name: agent.name,
        description: agent.description,
        system_message: agent.system_message,
        ability_ids: request.ability_ids,
        created_at: agent.created_at,
        updated_at: agent.updated_at,
    })
}

/// Update agent by id.
///
/// # Errors
///
/// Returns error if agent with given id does not exist or there was an error
/// while accessing database.
#[tauri::command]
pub async fn update_agent(request: UpdateAgent, pool_mutex: State<'_, DbMutex>) -> Result<Agent> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let now = Utc::now();
    query!(
        r#"
        UPDATE agents
        SET name = $2, description = $3, system_message = $4, updated_at = $5
        WHERE id = $1
        "#,
        request.id,
        request.name,
        request.description,
        request.system_message,
        now,
    )
    .execute(&mut *tx)
    .await
    .with_context(|| "Failed to update agent")?;

    // TODO(ri-nat): Be more clever here
    query!(
        "DELETE FROM agent_abilities WHERE agent_id = $1",
        request.id,
    )
    .execute(&mut *tx)
    .await
    .with_context(|| "Failed to delete agent abilities")?;

    for ability_id in &request.ability_ids {
        query!(
            "INSERT INTO agent_abilities (agent_id, ability_id) VALUES ($1, $2)",
            request.id,
            ability_id,
        )
        .execute(&mut *tx)
        .await
        .with_context(|| "Failed to insert agent ability")?;
    }

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    let agent = query_as!(
        AgentRow,
        r#"
        SELECT id, name, description, system_message, created_at, updated_at
        FROM agents
        WHERE id = $1
        "#,
        request.id,
    )
    .fetch_one(pool)
    .await
    .with_context(|| "Failed to fetch agent")?;

    Ok(Agent {
        id: agent.id,
        name: agent.name,
        description: agent.description,
        system_message: agent.system_message,
        ability_ids: request.ability_ids,
        created_at: agent.created_at,
        updated_at: agent.updated_at,
    })
}

/// Delete agent by id.
///
/// # Errors
///
/// Returns error if agent with given id does not exist.
/// Returns error if any error occurs during transaction.
#[tauri::command]
pub async fn delete_agent(request: DeleteAgent, pool_mutex: State<'_, DbMutex>) -> Result<()> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    query!(
        "DELETE FROM agent_abilities WHERE agent_id = $1",
        request.id
    )
    .execute(&mut *tx)
    .await
    .with_context(|| "Failed to delete agent abilities")?;

    query!("DELETE FROM agents WHERE id = $1", request.id)
        .execute(&mut *tx)
        .await
        .with_context(|| "Failed to delete agent")?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}
