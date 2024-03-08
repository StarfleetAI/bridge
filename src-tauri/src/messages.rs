// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("chunk deserialization error: {0}")]
    ChunkDeserialization(#[from] serde_json::Error),
    #[error("no tool calls found in message `{0}`")]
    NoToolCallsFound(i64),
    #[error("tool call has no id in message `{0}`")]
    NoToolCallId(i64),
}
