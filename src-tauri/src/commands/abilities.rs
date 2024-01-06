// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// TODO(ri-nat): I don't really know, why Clippy is mad about these here, but let make him quiet for now.
#![allow(clippy::used_underscore_binding)]

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use tauri::State;

use crate::types::{DbMutex, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ability {
    id: i64,
    name: String,
    description: String,
    code: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    pub abilities: Vec<Ability>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Create {
    pub name: String,
    pub description: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Delete {
    pub id: i64,
}

/// List all abilities.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_abilities(pool_mutex: State<'_, DbMutex>) -> Result<List> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    let abilities = query_as!(
        Ability,
        "SELECT id, name, description, code, created_at, updated_at FROM abilities ORDER BY id DESC"
    )
    .fetch_all(pool)
    .await
    .with_context(|| "Failed to fetch abilities")?;

    Ok(List { abilities })
}

/// Create new ability.
///
/// # Errors
///
/// Returns error if there was a problem while inserting new ability.
#[tauri::command]
pub async fn create_ability(request: Create, pool_mutex: State<'_, DbMutex>) -> Result<Ability> {
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
pub async fn update_ability(request: Update, pool_mutex: State<'_, DbMutex>) -> Result<Ability> {
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
pub async fn delete_ability(request: Delete, pool_mutex: State<'_, DbMutex>) -> Result<()> {
    let pool_guard = pool_mutex.lock().await;
    let pool = pool_guard.as_ref().with_context(|| "Failed to get pool")?;

    query!("DELETE FROM abilities WHERE id = $1", request.id)
        .execute(pool)
        .await
        .with_context(|| "Failed to delete ability")?;

    Ok(())
}
