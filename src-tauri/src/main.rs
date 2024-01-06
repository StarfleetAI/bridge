// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Context;
use dotenv::dotenv;
use env_logger::{Builder, Env};
use log::info;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use tauri::{generate_handler, Manager, State};

use bridge::{
    commands,
    types::{DbMutex, Result},
};
use tokio::sync::Mutex;

fn main() -> Result<()> {
    dotenv().ok();
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let db_state: DbMutex = Mutex::new(None);

    info!("Starting Bridge...");
    tauri::Builder::default()
        .manage(db_state)
        .invoke_handler(generate_handler![
            commands::abilities::list_abilities,
            commands::abilities::create_ability,
            commands::abilities::update_ability,
            commands::abilities::delete_ability,
        ])
        .setup(setup_handler)
        .run(tauri::generate_context!())
        .with_context(|| "Failed to run tauri application")?;

    Ok(())
}

fn setup_handler(app: &mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle();

    let database_url = if let Ok(url) = std::env::var("DATABASE_URL") {
        url
    } else {
        info!("DATABASE_URL not set, using default");
        format!(
            "{}/db.sqlite3",
            app_handle
                .path_resolver()
                .app_local_data_dir()
                .expect("Failed to get app local data dir for database")
                .to_str()
                .expect("Failed to convert app local data dir to string")
        )
    };

    let db_url = database_url.clone();
    tauri::async_runtime::spawn(async move {
        if !Sqlite::database_exists(&db_url)
            .await
            .with_context(|| "Failed to check if database exists")?
        {
            info!("No database found, creating one");
            Sqlite::create_database(&db_url)
                .await
                .with_context(|| "Failed to create database")?;
        }

        Ok::<(), anyhow::Error>(())
    });

    info!("Connecting to a database");
    let pool = tauri::async_runtime::block_on(async move {
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .with_context(|| "Failed to connect to sqlite")
    })?;

    info!("Running migrations");
    tauri::async_runtime::block_on(async {
        sqlx::migrate!("db/migrations")
            .run(&pool)
            .await
            .with_context(|| "Failed to run migrations")
    })?;

    let db_state: State<Mutex<Option<Pool<Sqlite>>>> = app_handle.state();
    let mut dbs = db_state.blocking_lock();
    *dbs = Some(pool);

    Ok(())
}
