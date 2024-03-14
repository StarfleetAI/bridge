// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

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
    Executor(#[from] crate::task_executor::Error),
    #[error(transparent)]
    Messages(#[from] crate::messages::Error),
    #[error(transparent)]
    Settings(#[from] crate::settings::Error),

    #[error("ability is used by agents")]
    AbilityIsUsedByAgents,
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(format!("{self:#}").as_str())
    }
}
