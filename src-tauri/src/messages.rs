// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("chunk deserialization error: {0}")]
    ChunkDeserialization(#[from] serde_json::Error),
    #[error("no valid chunk prefix found")]
    NoValidChunkPrefix,
    #[error("no tool calls found in message")]
    NoToolCallsFound,
    #[error("tool call has no `id`")]
    NoToolCallId,
}
