// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Context;
use bridge_common::{channel::Channel, repo};
use dotenvy::dotenv;
use tauri::{async_runtime::block_on, generate_handler, App, LogicalSize, Manager};
use tokio::sync::RwLock;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

use bridge::{channel::TauriChannel, commands, database, task_executor, types::Result};

fn main() -> Result<()> {
    let _ = fix_path_env::fix();
    dotenv().ok();

    let format = fmt::format();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .event_format(format)
        .init();

    // tauri_plugin_deep_link::prepare("com.starfleetai.bridge");

    info!("Starting Bridge...");
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            commands::abilities::create_ability,
            commands::abilities::delete_ability,
            commands::abilities::list_abilities,
            commands::abilities::update_ability,
            commands::agents_chats::list_agents_chats,
            commands::agents::create_agent,
            commands::agents::delete_agent,
            commands::agents::list_agents,
            commands::agents::update_agent_is_enabled,
            commands::agents::update_agent,
            commands::chats::create_chat,
            commands::chats::delete_chat,
            commands::chats::get_chat,
            commands::chats::list_chats,
            commands::chats::toggle_chat_is_pinned,
            commands::chats::update_chat_model_full_name,
            commands::chats::update_chat_title,
            commands::messages::approve_tool_call,
            commands::messages::create_message,
            commands::messages::delete_message,
            commands::messages::deny_tool_call,
            commands::messages::get_raw_message_content,
            commands::messages::list_messages,
            commands::messages::update_message_content,
            commands::models::list_models,
            commands::pages::create_page,
            commands::pages::delete_page,
            commands::pages::get_page,
            commands::pages::list_pages,
            commands::pages::update_page,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::task_results::get_task_result_text_data,
            commands::task_results::list_task_results,
            commands::tasks::create_task,
            commands::tasks::delete_task,
            commands::tasks::duplicate_task,
            commands::tasks::execute_task,
            commands::tasks::get_task,
            commands::tasks::list_child_tasks,
            commands::tasks::list_root_tasks_by_status,
            commands::tasks::list_root_tasks,
            commands::tasks::plan_task,
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
    let app_local_data_dir = app_handle
        .path_resolver()
        .app_local_data_dir()
        .expect("Failed to get app local data dir")
        .to_str()
        .expect("Failed to convert app local data dir to string")
        .to_string();

    // Create `app_local_data_dir` if it doesn't exist
    std::fs::create_dir_all(&app_local_data_dir)
        .with_context(|| format!("Failed to create app local data dir: {app_local_data_dir}"))?;

    let channel: Channel = Box::new(TauriChannel::new(app_handle.clone()));
    app_handle.manage(channel);

    set_main_window_min_size(app)?;

    let pool = block_on(async { bridge_common::database::new_pool().await })?;
    block_on(async { database::seed(&pool).await })?;

    let settings = block_on(async { repo::settings::get(&pool, bridge::CID).await })?;
    app_handle.manage(RwLock::new(settings));

    app_handle.manage(pool);

    block_on(async { task_executor::start_loop(&app_handle).await });

    info!("Startup sequence completed!");
    info!("Launching Bridge! 🚀");

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
