// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// TODO(ri-nat): I don't really know, why Clippy is mad about these here, but let make him quiet for now.
#![allow(clippy::used_underscore_binding)]

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use log::debug;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, query_scalar};
use tauri::State;
use tokio::{process::Command, sync::RwLock};

use crate::{
    clients::openai::{Function, Tool},
    errors::Error,
    settings::Settings,
    types::{DbMutex, Result},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ability {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub code: String,
    pub parameters_json: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AbilitiesList {
    pub abilities: Vec<Ability>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAbility {
    pub name: String,
    pub description: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAbility {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteAbility {
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFunctionParameters {
    pub code: String,
}

/// Get function parameters by code.
///
/// # Errors
///
/// Returns error if there was a problem when determining function parameters.
// TODO: work correctly if there are imports in the code
pub async fn get_function_parameters(code: &str, python_path: &str) -> Result<Function> {
    let output = Command::new(python_path)
        .arg("-c")
        .arg(format!(
            r#"
import json
from typing import Annotated
from bridge import Agent

agent = Agent(name='')

@agent.register(description='')
{code}

print(json.dumps(agent.functions_definitions()[0]))
"#
        ))
        .output()
        .await
        .with_context(|| "Failed to execute python script")?;

    debug!("Function definitions script output: {:?}", output);

    let tool: Tool = serde_json::from_slice(&output.stdout)
        .with_context(|| "Failed to parse python script output")?;

    Ok(tool.function)
}

/// Preprocess code: trim leading and trailing whitespaces around the code, remove trailing whitespaces
/// from each line.
fn preprocess_code(code: &str) -> String {
    let mut result = String::new();
    for line in code.lines() {
        result.push_str(line.trim_end());
        result.push('\n');
    }
    result.trim().to_string()
}

/// List all abilities.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_abilities(pool_mutex: State<'_, DbMutex>) -> Result<AbilitiesList> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let abilities = query_as!(
        Ability,
        "SELECT id, name, description, code, parameters_json, created_at, updated_at FROM abilities ORDER BY id DESC"
    )
    .fetch_all(pool)
    .await
    .with_context(|| "Failed to fetch abilities")?;

    Ok(AbilitiesList { abilities })
}

/// Create new ability.
///
/// # Errors
///
/// Returns error if there was a problem while inserting new ability.
#[tauri::command]
pub async fn create_ability(
    request: CreateAbility,
    pool_mutex: State<'_, DbMutex>,
    settings: State<'_, RwLock<Settings>>,
) -> Result<Ability> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let code = preprocess_code(&request.code);

    let settings_guard = settings.read().await;
    let params = match &settings_guard.python_path {
        Some(path) => get_function_parameters(&code, path)
            .await
            .with_context(|| format!("Failed to get function parameters for code: {code}"))?,
        None => {
            return Err(anyhow::anyhow!("Python path is not set").into());
        }
    };

    let params_json = serde_json::to_string(&params)
        .with_context(|| "Failed to serialize function parameters to json")?;

    let now = Utc::now();
    let ability = query_as!(
        Ability,
        r#"
        INSERT INTO abilities (name, description, code, parameters_json, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4, $5)
        RETURNING id, name, description, code, parameters_json, created_at, updated_at
        "#,
        request.name,
        request.description,
        code,
        params_json,
        now
    )
    .fetch_one(pool)
    .await
    .with_context(|| "Failed to create ability")?;

    Ok(ability)
}

/// Update ability by id.
///
/// # Errors
///
/// Returns error if ability with given id does not exist or there was an error
/// while accessing database.
#[tauri::command]
pub async fn update_ability(
    request: UpdateAbility,
    pool_mutex: State<'_, DbMutex>,
    settings: State<'_, RwLock<Settings>>,
) -> Result<Ability> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let code = preprocess_code(&request.code);

    let settings_guard = settings.read().await;
    let params = match &settings_guard.python_path {
        Some(path) => get_function_parameters(&code, path)
            .await
            .with_context(|| format!("Failed to get function parameters for code: {code}"))?,
        None => {
            return Err(anyhow::anyhow!("Python path is not set").into());
        }
    };

    let params_json = serde_json::to_string(&params)
        .with_context(|| "Failed to serialize function parameters to json")?;

    let now = Utc::now();
    query!(
        r#"
        UPDATE abilities
        SET name = $2, description = $3, code = $4, parameters_json = $5, updated_at = $6
        WHERE id = $1
        "#,
        request.id,
        request.name,
        request.description,
        code,
        params_json,
        now
    )
    .execute(pool)
    .await
    .with_context(|| "Failed to update ability")?;

    // Fetching updated record that way because using `RETURNING` clause
    // above leads to getting Option<i64> instead of i64.
    let ability = query_as!(
        Ability,
        r#"
        SELECT id, name, description, code, parameters_json, created_at, updated_at
        FROM abilities
        WHERE id = $1
        "#,
        request.id
    )
    .fetch_one(pool)
    .await
    .with_context(|| "Failed to fetch ability")?;

    Ok(ability)
}

/// Delete ability by id.
///
/// # Errors
///
/// Returns error if ability with given id does not exist.
#[tauri::command]
pub async fn delete_ability(request: DeleteAbility, pool_mutex: State<'_, DbMutex>) -> Result<()> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;
    let agents_count: i32 = query_scalar!(
        "SELECT COUNT(*) FROM agent_abilities WHERE ability_id = $1",
        request.id
    )
    .fetch_one(&mut *tx)
    .await
    .with_context(|| "Failed to count agents")?;

    if agents_count > 0 {
        return Err(Error::AbilityIsUsedByAgents);
    }

    query!("DELETE FROM abilities WHERE id = $1", request.id)
        .execute(&mut *tx)
        .await
        .with_context(|| "Failed to delete ability")?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}
