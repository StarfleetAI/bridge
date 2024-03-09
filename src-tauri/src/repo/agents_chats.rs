// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use anyhow::Context;
use sqlx::{query, query_as, Executor, Sqlite};

use crate::types::Result;

pub struct AgentsChat {
    pub agent_id: i64,
    pub chat_id: i64,
}

/// List all agents for chat.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn list<'a, E>(executor: E) -> Result<HashMap<i64, Vec<i64>>>
where
    E: Executor<'a, Database = Sqlite>,
{
    let rows: Vec<AgentsChat> = query_as!(AgentsChat, "SELECT * FROM agents_chats")
        .fetch_all(executor)
        .await
        .with_context(|| "Failed to fetch agents for chat")?;

    let mut chat_agents: HashMap<i64, Vec<i64>> = HashMap::new();

    for row in rows {
        chat_agents
            .entry(row.chat_id)
            .or_insert_with(Vec::new)
            .push(row.agent_id);
    }

    Ok(chat_agents)
}

/// Add agent to chat.
///
/// # Errors
///
/// Returns error if there was a problem while creating `agents_chats` record.
pub async fn create<'a, E>(executor: E, agent_id: i64, chat_id: i64) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    query!(
        "INSERT INTO agents_chats (agent_id, chat_id) VALUES ($1, $2)",
        agent_id,
        chat_id
    )
    .execute(executor)
    .await
    .with_context(|| "Failed to create `agents_chats` record")?;

    Ok(())
}

/// Remove agents from chat.
///
/// # Errors
///
/// Returns error if there was a problem while deleting `agents_chats` records.
pub async fn delete_for_chat<'a, E>(executor: E, chat_id: i64) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    query!("DELETE FROM agents_chats WHERE chat_id = $1", chat_id)
        .execute(executor)
        .await
        .with_context(|| "Failed to delete `agents_chats` records")?;

    Ok(())
}
