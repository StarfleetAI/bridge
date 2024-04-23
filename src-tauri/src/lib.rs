// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use lazy_static::lazy_static;

pub mod channel;
pub mod commands;
pub mod database;
pub mod errors;
pub mod messages;
pub mod task_executor;
pub mod types;

/// Virtual company ID for local database.
pub const CID: i32 = 0;
/// Virtual user ID for local database.
pub const UID: i32 = 0;

lazy_static! {
    static ref USER_AGENT: String = format!("StarfleetAI-Bridge/{}", env!("CARGO_PKG_VERSION"));
}
