// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use crate::messages::Error as MessagesError;
use crate::settings::Error as SettingsError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    DatabaseMigrate(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error(transparent)]
    Messages(#[from] MessagesError),
    #[error(transparent)]
    Settings(#[from] SettingsError),

    #[error("ability is used by agents")]
    AbilityIsUsedByAgents,
    #[error("no `tool_calls` found in message")]
    NoToolCallsFound,
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(format!("{self:#}").as_str())
    }
}
