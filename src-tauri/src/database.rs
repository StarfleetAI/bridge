// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use sqlx::{Pool, Postgres};
use tracing::debug;

use crate::types::Result;

/// Seed the database with initial data
///
/// # Errors
///
/// This function will return an error if any of the queries fail to execute.
pub async fn seed(pool: &Pool<Postgres>) -> Result<()> {
    debug!("Seeding the database");
    for query in seed_queries() {
        sqlx::query(query).execute(pool).await?;
    }

    Ok(())
}

fn seed_queries() -> Vec<&'static str> {
    include_str!("../db/seeds.sql")
        .split(';')
        .map(str::trim)
        .filter(|q| !q.is_empty())
        .collect()
}
