// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use std::path::PathBuf;
use tracing::{debug, info};

use crate::repo::{messages, tasks};
use crate::types::Result;

/// Create a new database pool.
///
/// # Errors
///
/// Will return an error if the database URL can't be read, the database doesn't exist and can't be
/// created, or if there was a problem while connecting to the database.
///
/// # Panics
///
/// Will panic if the database path can't be converted to a string.
pub async fn new_pool(app_local_data_dir: &str) -> Result<Pool<Sqlite>> {
    let database_url = if let Ok(url) = std::env::var("DATABASE_URL") {
        debug!("Using DATABASE_URL: {}", url);
        url
    } else {
        debug!("DATABASE_URL not set, using default DB location");

        let mut path = PathBuf::new();
        path.push(app_local_data_dir);
        path.push("db.sqlite3");
        path.into_os_string()
            .into_string()
            .expect("Failed to convert database path to string")
    };

    let db_url = database_url.clone();
    if !Sqlite::database_exists(&db_url)
        .await
        .with_context(|| "Failed to check if database exists")?
    {
        info!("No database found, creating one");
        Sqlite::create_database(&db_url)
            .await
            .with_context(|| "Failed to create database")?;
    }

    info!("Connecting to a database");

    Ok(SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .with_context(|| "Failed to connect to sqlite")?)
}

/// Prepare the database by running migrations and cleaning up after possible previous termination.
///
/// # Errors
///
/// Will return an error if the migrations can't be run or if there was a problem while cleaning up
/// after possible previous termination.
pub async fn prepare(pool: &Pool<Sqlite>) -> Result<()> {
    debug!("Running migrations");
    sqlx::migrate!("db/migrations")
        .run(pool)
        .await
        .with_context(|| "Failed to run migrations")?;

    debug!("Cleaning up after possible previous termination");
    messages::transition_all(pool, messages::Status::Writing, messages::Status::Failed).await?;
    tasks::transition_all(pool, tasks::Status::InProgress, tasks::Status::ToDo).await?;

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
