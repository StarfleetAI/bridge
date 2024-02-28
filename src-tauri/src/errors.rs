// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

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

    #[error("Settings error: {0}")]
    SettingsError(#[from] SettingsError),

    #[error("Ability is used by agents")]
    AbilityIsUsedByAgents,
    #[error("No `tool_calls` found in message")]
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
