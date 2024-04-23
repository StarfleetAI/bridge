// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use anyhow::Context;
use bridge_common::{
    abilities::{get_function_definition, preprocess_code},
    repo,
    types::abilities::Ability,
};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::types::{DbPool, Result};

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
    pub id: i32,
    pub name: String,
    pub description: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFunctionParameters {
    pub code: String,
}

/// List all abilities.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_abilities(pool: State<'_, DbPool>) -> Result<AbilitiesList> {
    let abilities = repo::abilities::list(&*pool, crate::CID).await?;

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

    let parameters_json = get_function_definition(&code)
        .await
        .with_context(|| format!("Failed to get function parameters for code: {code}"))?;

    let ability = repo::abilities::create(
        &*pool,
        crate::CID,
        repo::abilities::CreateParams {
            name: request.name,
            description: request.description,
            code,
            parameters_json,
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

    let parameters_json = get_function_definition(&code)
        .await
        .with_context(|| format!("Failed to get function parameters for code: {code}"))?;

    let ability = repo::abilities::update(
        &*pool,
        crate::CID,
        repo::abilities::UpdateParams {
            id: request.id,
            name: request.name,
            description: request.description,
            code,
            parameters_json,
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
pub async fn delete_ability(id: i32, pool: State<'_, DbPool>) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .with_context(|| "Failed to begin transaction")?;

    let agents_count = repo::agent_abilities::get_agents_count(&mut *tx, crate::CID, id).await?;

    if agents_count > 0 {
        return Err(crate::errors::Error::from(
            bridge_common::errors::Error::from(bridge_common::abilities::Error::IsUsedByAgents),
        ));
    }

    repo::abilities::delete(&mut *tx, crate::CID, id).await?;

    tx.commit()
        .await
        .with_context(|| "Failed to commit transaction")?;

    Ok(())
}
