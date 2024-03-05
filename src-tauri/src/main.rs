// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Context;
use dotenvy::dotenv;
use tauri::{async_runtime::block_on, generate_handler, App, LogicalSize, Manager};
use tokio::sync::RwLock;
use tracing::{debug, info};

use bridge::{commands, database, settings::Settings, types::Result};
use tracing_subscriber::{fmt, EnvFilter};

fn main() -> Result<()> {
    dotenv().ok();

    let format = fmt::format();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .event_format(format)
        .init();

    tauri_plugin_deep_link::prepare("com.starfleetai.bridge");

    info!("Starting Bridge...");
    tauri::Builder::default()
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
            commands::chats::update_chat_title,
            commands::messages::approve_tool_call,
            commands::messages::create_message,
            commands::messages::delete_message,
            commands::messages::deny_tool_call,
            commands::messages::list_messages,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::tasks::cancel_task,
            commands::tasks::create_task,
            commands::tasks::delete_task,
            commands::tasks::execute_task,
            commands::tasks::get_task,
            commands::tasks::list_child_tasks,
            commands::tasks::list_root_tasks,
            commands::tasks::pause_task,
            commands::tasks::revise_task,
            commands::tasks::update_task,
        ])
        .setup(setup_handler)
        .run(tauri::generate_context!())
        .with_context(|| "Failed to run tauri application")?;

    Ok(())
}

// We need to resolve a local_data_dir in order to create a DB file. The easiest way to do this is
// using the setup_handler, but it can't be async, so we need to spawn a task to do the actual
// work.
fn setup_handler(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
    let app_local_data_dir = app_handle
        .path_resolver()
        .app_local_data_dir()
        .expect("Failed to get app local data dir")
        .to_str()
        .expect("Failed to convert app local data dir to string")
        .to_string();

    let settings = block_on(async { Settings::load_from_disk(&app_local_data_dir).await })?;
    app_handle.manage(RwLock::new(settings));

    set_main_window_min_size(app)?;

    let pool = block_on(async { database::new_pool(&app_local_data_dir).await })?;

    block_on(async { database::prepare(&pool).await })?;

    app_handle.manage(pool);

    info!("Startup sequence completed!");
    info!("Launching! 🚀");

    Ok(())
}

fn set_main_window_min_size(app: &App) -> Result<()> {
    let main_window = app
        .get_window("main")
        .with_context(|| "Failed to get main window")?;

    let logical_size: LogicalSize<i32> = [420, 690].into();

    main_window
        .set_min_size(Some(
            logical_size.to_physical::<i32>(
                main_window
                    .scale_factor()
                    .with_context(|| "Failed to get scale factor")?,
            ),
        ))
        .with_context(|| "Failed to set min window size")?;

    Ok(())
}
