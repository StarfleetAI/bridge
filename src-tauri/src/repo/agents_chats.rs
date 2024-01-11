// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use sqlx::{query, Executor, Sqlite};

use crate::types::Result;

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
