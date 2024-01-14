// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use sqlx::{Pool, Sqlite};

pub type Result<T> = std::result::Result<T, crate::errors::Error>;

pub type DbPool = Pool<Sqlite>;
