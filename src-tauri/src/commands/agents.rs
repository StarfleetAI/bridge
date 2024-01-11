// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// TODO(ri-nat): I don't really know, why Clippy is mad about these here, but let make him quiet for now.
#![allow(clippy::used_underscore_binding)]

use std::collections::BTreeMap;

use anyhow::Context;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    repo::{
        self,
        agents::{CreateParams, UpdateParams},
    },
    types::{DbMutex, Result},
};

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

    let rows = repo::agents::list(pool).await?;

    let ability_rows = repo::agent_abilities::list(pool).await?;

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

    let agent = repo::agents::create(
        &mut *tx,
        CreateParams {
            name: request.name,
            description: request.description,
            system_message: request.system_message,
        },
    )
    .await?;

    for ability_id in &request.ability_ids {
        repo::agent_abilities::create(&mut *tx, agent.id, *ability_id).await?;
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

    let agent = repo::agents::update(
        &mut *tx,
        UpdateParams {
            id: request.id,
            name: request.name,
            description: request.description,
            system_message: request.system_message,
        },
    )
    .await?;

    // TODO(ri-nat): Be more clever here
    repo::agent_abilities::delete_for_agent(&mut *tx, request.id).await?;
    for ability_id in &request.ability_ids {
        repo::agent_abilities::create(&mut *tx, request.id, *ability_id).await?;
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

    repo::agent_abilities::delete_for_agent(&mut *tx, request.id).await?;
    repo::agents::delete(&mut *tx, request.id).await?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}
