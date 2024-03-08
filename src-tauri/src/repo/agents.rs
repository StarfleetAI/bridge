// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use sqlx::{Executor, query, query_as, Sqlite};

use crate::types::Result;

pub struct Agent {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub system_message: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct CreateParams {
    pub name: String,
    pub description: String,
    pub system_message: String,
}

pub struct UpdateParams {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub system_message: String,
}

/// List all agents.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn list<'a, E>(executor: E) -> Result<Vec<Agent>>
where
    E: Executor<'a, Database = Sqlite>,
{
    let agents = query_as!(
        Agent,
        r#"
        SELECT *
        FROM agents
        ORDER BY agents.id ASC
        "#
    )
    .fetch_all(executor)
    .await
    .with_context(|| "Failed to list agents")?;

    Ok(agents)
}

/// Get agent by id.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn get<'a, E>(executor: E, id: i64) -> Result<Agent>
where
    E: Executor<'a, Database = Sqlite>,
{
    let agent = query_as!(
        Agent,
        r#"
        SELECT *
        FROM agents
        WHERE agents.id = $1
        LIMIT 1
        "#,
        id
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to get agent")?;

    Ok(agent)
}

/// Get agent by chat id.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn get_for_chat<'a, E>(executor: E, chat_id: i64) -> Result<Agent>
where
    E: Executor<'a, Database = Sqlite>,
{
    let agent = query_as!(
        Agent,
        r#"
        SELECT agents.*
        FROM agents
        INNER JOIN agents_chats ON agents.id = agents_chats.agent_id
        WHERE agents_chats.chat_id = $1
        LIMIT 1
        "#,
        chat_id
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to get agent")?;

    Ok(agent)
}

/// Get agent.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn create<'a, E>(executor: E, params: CreateParams) -> Result<Agent>
where
    E: Executor<'a, Database = Sqlite>,
{
    let now = Utc::now();
    let agent = query_as!(
        Agent,
        r#"
        INSERT INTO agents (name, description, system_message, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        RETURNING *
        "#,
        params.name,
        params.description,
        params.system_message,
        now,
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to create agent")?;

    Ok(agent)
}

/// Update agent.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn update<'a, E>(executor: E, params: UpdateParams) -> Result<Agent>
where
    E: Executor<'a, Database = Sqlite>,
{
    let now = Utc::now();
    let agent = query_as!(
        Agent,
        r#"
        UPDATE agents
        SET name = $2, description = $3, system_message = $4, updated_at = $5
        WHERE id = $1
        RETURNING id as "id!", name, description, system_message, created_at, updated_at
        "#,
        params.id,
        params.name,
        params.description,
        params.system_message,
        now,
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to update agent")?;

    Ok(agent)
}

/// Delete agent.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn delete<'a, E>(executor: E, id: i64) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    query!("DELETE FROM agents WHERE id = $1", id)
        .execute(executor)
        .await
        .with_context(|| "Failed to delete agent")?;

    Ok(())
}
