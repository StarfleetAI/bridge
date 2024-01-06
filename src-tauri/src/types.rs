// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use sqlx::{Pool, Sqlite};
use tokio::sync::Mutex;

pub type Result<T> = std::result::Result<T, crate::errors::Error>;

pub type DbMutex = Mutex<Option<Pool<Sqlite>>>;
