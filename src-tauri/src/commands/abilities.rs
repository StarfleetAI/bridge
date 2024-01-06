// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// TODO(ri-nat): I don't really know, why Clippy is mad about these here, but let make him quiet for now.
#![allow(clippy::used_underscore_binding)]

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, query_scalar};
use tauri::State;

use crate::{
    errors::Error,
    types::{DbMutex, Result},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ability {
    id: i64,
    name: String,
    description: String,
    code: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
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
        "SELECT id, name, description, code, created_at, updated_at FROM abilities ORDER BY id DESC"
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
) -> Result<Ability> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let now = Utc::now();
    let ability = query_as!(
        Ability,
        r#"
        INSERT INTO abilities (name, description, code, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        RETURNING id, name, description, code, created_at, updated_at
        "#,
        request.name,
        request.description,
        request.code,
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
) -> Result<Ability> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let now = Utc::now();
    query!(
        r#"
        UPDATE abilities
        SET name = $2, description = $3, code = $4, updated_at = $5
        WHERE id = $1
        "#,
        request.id,
        request.name,
        request.description,
        request.code,
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
        SELECT id, name, description, code, created_at, updated_at
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
