// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use std::collections::BTreeMap;

use anyhow::Context;
use bridge_common::repo::{
    self,
    agents::{CreateParams, UpdateParams},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::types::{DbPool, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Agent {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub system_message: String,
    pub ability_ids: Vec<i32>,
    pub is_enabled: bool,
    pub is_code_interpreter_enabled: bool,
    pub is_web_browser_enabled: bool,
    pub execution_steps_limit: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
    pub ability_ids: Vec<i32>,
    pub is_code_interpreter_enabled: bool,
    pub is_web_browser_enabled: bool,
    pub execution_steps_limit: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAgent {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub system_message: String,
    pub ability_ids: Vec<i32>,
    pub is_code_interpreter_enabled: bool,
    pub is_web_browser_enabled: bool,
    pub execution_steps_limit: Option<i32>,
}

/// List all agents.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_agents(pool: State<'_, DbPool>) -> Result<AgentsList> {
    let rows = repo::agents::list(&*pool, crate::CID).await?;

    let ability_rows = repo::agent_abilities::list(&*pool, crate::CID).await?;

    let mut abilities: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
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
            is_enabled: row.is_enabled,
            is_code_interpreter_enabled: row.is_code_interpreter_enabled,
            is_web_browser_enabled: row.is_web_browser_enabled,
            execution_steps_limit: row.execution_steps_limit,
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
pub async fn create_agent(request: CreateAgent, pool: State<'_, DbPool>) -> Result<Agent> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let agent = repo::agents::create(
        &mut *tx,
        crate::CID,
        CreateParams {
            name: request.name,
            description: request.description,
            system_message: request.system_message,
            is_code_interpreter_enabled: request.is_code_interpreter_enabled,
            is_web_browser_enabled: request.is_web_browser_enabled,
        },
    )
    .await?;

    for ability_id in &request.ability_ids {
        repo::agent_abilities::create(&mut *tx, crate::CID, agent.id, *ability_id).await?;
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
        is_enabled: agent.is_enabled,
        is_code_interpreter_enabled: request.is_code_interpreter_enabled,
        is_web_browser_enabled: request.is_web_browser_enabled,
        execution_steps_limit: request.execution_steps_limit,
        created_at: agent.created_at,
        updated_at: agent.updated_at,
    })
}

/// Update `is_enabled` field for agent by id.
///
/// # Errors
///
/// Returns error if any error occurs while accessing database.
#[tauri::command]
pub async fn update_agent_is_enabled(
    id: i32,
    is_enabled: bool,
    pool: State<'_, DbPool>,
) -> Result<()> {
    repo::agents::update_is_enabled(&*pool, crate::CID, id, is_enabled).await?;

    Ok(())
}

/// Update agent by id.
///
/// # Errors
///
/// Returns error if agent with given id does not exist or there was an error
/// while accessing database.
#[tauri::command]
pub async fn update_agent(request: UpdateAgent, pool: State<'_, DbPool>) -> Result<Agent> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let agent = repo::agents::update(
        &mut *tx,
        crate::CID,
        UpdateParams {
            id: request.id,
            name: request.name,
            description: request.description,
            system_message: request.system_message,
            is_code_interpreter_enabled: request.is_code_interpreter_enabled,
            is_web_browser_enabled: request.is_web_browser_enabled,
        },
    )
    .await?;

    // TODO(ri-nat): Be more clever here
    repo::agent_abilities::delete_for_agent(&mut *tx, crate::CID, request.id).await?;
    for ability_id in &request.ability_ids {
        repo::agent_abilities::create(&mut *tx, crate::CID, request.id, *ability_id).await?;
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
        is_enabled: agent.is_enabled,
        is_code_interpreter_enabled: agent.is_code_interpreter_enabled,
        is_web_browser_enabled: agent.is_web_browser_enabled,
        execution_steps_limit: agent.execution_steps_limit,
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
pub async fn delete_agent(id: i32, pool: State<'_, DbPool>) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    repo::agent_abilities::delete_for_agent(&mut *tx, crate::CID, id).await?;
    repo::agents::delete(&mut *tx, crate::CID, id).await?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}
