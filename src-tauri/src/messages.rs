// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("chunk deserialization error: {0}")]
    ChunkDeserialization(#[from] serde_json::Error),
}
