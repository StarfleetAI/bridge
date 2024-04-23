// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    #[error(transparent)]
    TokioJoin(#[from] tokio::task::JoinError),
    #[error(transparent)]
    Common(#[from] bridge_common::errors::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(format!("{self:#}").as_str())
    }
}
