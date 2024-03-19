// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use anyhow::Context;
use askama::Template;
use serde::{Deserialize, Serialize};
use tauri::State;
use tracing::debug;

use crate::{
    clients::openai::{Function, Tool},
    docker,
    errors::Error,
    repo::{self, abilities::Ability},
    types::{DbPool, Result},
};

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
pub struct GetFunctionParameters {
    pub code: String,
}

#[derive(Template)]
#[template(path = "python/get_function_definition.py", escape = "none")]
struct GetFunctionDefinitionTemplate<'a> {
    code: &'a str,
}

/// Get function definition by its code.
///
/// # Errors
///
/// Returns error if there was a problem when determining function parameters.
// TODO: work correctly if there are imports in the code
pub async fn get_function_definition(code: &str) -> Result<Function> {
    let template = GetFunctionDefinitionTemplate { code };

    // TODO: seems a little bit inefficient to run a container only to get a function definition.
    //       Consider using some Python parser library to get type hints on a Rust side.
    let output = docker::run_python_code(
        &template
            .render()
            .context("Failed to render `get_function_definition` script")?,
    )
    .await?;

    debug!("Function definition script output: {:?}", output);

    let tool: Tool = serde_json::from_str(&output)
        .with_context(|| "Failed to parse function definition script output")?;

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
pub async fn list_abilities(pool: State<'_, DbPool>) -> Result<AbilitiesList> {
    let abilities = repo::abilities::list(&*pool).await?;

    Ok(AbilitiesList { abilities })
}

/// Create new ability.
///
/// # Errors
///
/// Returns error if there was a problem while inserting new ability.
#[tauri::command]
pub async fn create_ability(request: CreateAbility, pool: State<'_, DbPool>) -> Result<Ability> {
    let code = preprocess_code(&request.code);

    let params = get_function_definition(&code)
        .await
        .with_context(|| format!("Failed to get function parameters for code: {code}"))?;

    let params_json = serde_json::to_string(&params)
        .with_context(|| "Failed to serialize function parameters to json")?;

    let ability = repo::abilities::create(
        &*pool,
        repo::abilities::CreateParams {
            name: request.name,
            description: request.description,
            code,
            parameters_json: params_json,
        },
    )
    .await?;

    Ok(ability)
}

/// Update ability by id.
///
/// # Errors
///
/// Returns error if ability with given id does not exist or there was an error
/// while accessing database.
#[tauri::command]
pub async fn update_ability(request: UpdateAbility, pool: State<'_, DbPool>) -> Result<Ability> {
    let code = preprocess_code(&request.code);

    let params = get_function_definition(&code)
        .await
        .with_context(|| format!("Failed to get function parameters for code: {code}"))?;

    let params_json = serde_json::to_string(&params)
        .with_context(|| "Failed to serialize function parameters to json")?;

    let ability = repo::abilities::update(
        &*pool,
        repo::abilities::UpdateParams {
            id: request.id,
            name: request.name,
            description: request.description,
            code,
            parameters_json: params_json,
        },
    )
    .await?;

    Ok(ability)
}

/// Delete ability by id.
///
/// # Errors
///
/// Returns error if ability with given id does not exist.
#[tauri::command]
pub async fn delete_ability(id: i64, pool: State<'_, DbPool>) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let agents_count = repo::agent_abilities::get_agents_count(&mut *tx, id).await?;

    if agents_count > 0 {
        return Err(Error::AbilityIsUsedByAgents);
    }

    repo::abilities::delete(&mut *tx, id).await?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}
