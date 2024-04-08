// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("There aren't any data for update")]
    EmptyDataForUpdate,
}
