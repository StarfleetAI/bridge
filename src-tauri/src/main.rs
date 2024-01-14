// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use anyhow::Context;
use dotenvy::dotenv;
use env_logger::{Builder, Env};
use log::{debug, info};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite};
use tauri::{generate_handler, Manager};

use bridge::{commands, settings::Settings, types::Result};
use tokio::sync::RwLock;

fn main() -> Result<()> {
    dotenv().ok();
    Builder::from_env(Env::default().default_filter_or("info")).init();

    tauri_plugin_deep_link::prepare("com.starfleetai.bridge");

    info!("Starting Bridge...");
    tauri::Builder::default()
        .manage(RwLock::new(Settings::new()))
        .invoke_handler(generate_handler![
            commands::abilities::create_ability,
            commands::abilities::delete_ability,
            commands::abilities::list_abilities,
            commands::abilities::update_ability,
            commands::agents::create_agent,
            commands::agents::delete_agent,
            commands::agents::list_agents,
            commands::agents::update_agent,
            commands::chats::create_chat,
            commands::chats::delete_chat,
            commands::chats::get_chat,
            commands::chats::list_chats,
            commands::messages::approve_tool_call,
            commands::messages::create_message,
            commands::messages::delete_message,
            commands::messages::list_messages,
        ])
        .setup(setup_handler)
        .run(tauri::generate_context!())
        .with_context(|| "Failed to run tauri application")?;

    Ok(())
}

// We need to resolve a local_data_dir in order to create a DB file. The easiest way to do this is
// using the setup_handler, but it can't be async, so we need to spawn a task to do the actual
// work.
fn setup_handler(app: &mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle();

    tauri_plugin_deep_link::register("starfleetai-bridge", move |request| {
        debug!("Received deep link: {}", request);
        app_handle
            .emit_all("scheme-request-received", request)
            .unwrap();
    })
    .with_context(|| "Failed to register deep link handler")?;

    #[cfg(not(target_os = "macos"))]
    if let Some(url) = std::env::args().nth(1) {
        app.emit_all("scheme-request-received", url).unwrap();
    }

    let app_handle = app.handle();

    // TODO: read `database_url` from `Settings` instead of env
    let database_url = if let Ok(url) = std::env::var("DATABASE_URL") {
        url
    } else {
        info!("DATABASE_URL not set, using default");

        let mut path = PathBuf::new();
        path.push(
            app_handle
                .path_resolver()
                .app_local_data_dir()
                .expect("Failed to get app local data dir for database")
                .to_str()
                .expect("Failed to convert app local data dir to string"),
        );
        path.push("db.sqlite3");
        path.into_os_string()
            .into_string()
            .expect("Failed to convert database path to string")
    };

    let db_url = database_url.clone();
    tauri::async_runtime::block_on(async move {
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
    })?;

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

    app_handle.manage(pool);

    info!("Startup sequence completed!");
    info!("Launching! 🚀");

    Ok(())
}
