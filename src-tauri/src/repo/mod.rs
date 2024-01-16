// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

pub mod abilities;
pub mod agent_abilities;
pub mod agents;
pub mod agents_chats;
pub mod chats;
pub mod messages;
pub mod tasks;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub page: i64,
    pub per_page: i64,
}
