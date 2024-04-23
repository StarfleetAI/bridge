// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use std::collections::HashMap;

use bridge_common::repo;
use tauri::State;

use crate::{types::DbPool, types::Result};

/// List agents for chats.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn list_agents_chats(pool: State<'_, DbPool>) -> Result<HashMap<i32, Vec<i32>>> {
    Ok(repo::agents_chats::list(&*pool, crate::CID).await?)
}
