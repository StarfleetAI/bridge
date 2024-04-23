// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use bridge_common::task_executor;
use bridge_common::{channel::Channel, settings::Settings};
use tauri::{AppHandle, Manager, State};
use tokio::spawn;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tracing::{debug, error, info, instrument, trace};

use crate::channel::TauriChannel;
use crate::types::DbPool;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no root tasks to execute")]
    NoRootTasks,
    #[error("chat #{0} is not an execution chat")]
    NotAnExecutionChat(i64),
}

// TODO: implement graceful shutdown
#[instrument(skip_all)]
pub async fn start_loop(app_handle: &AppHandle) {
    let settings_state: State<'_, RwLock<Settings>> = app_handle.state();
    let settings = settings_state.read().await;

    let pool: State<'_, DbPool> = app_handle.state();
    let app_local_data_dir = app_handle
        .path_resolver()
        .app_local_data_dir()
        .expect("Failed to get app local data dir");

    info!(
        "Starting task execution loop with concurrency = {}",
        settings.tasks.execution_concurrency
    );

    for i in 0..settings.tasks.execution_concurrency {
        let settings = settings.clone();
        let pool = pool.inner().clone();
        let channel: Channel = Box::new(TauriChannel::new(app_handle.clone()));
        let app_local_data_dir = app_local_data_dir.clone();

        spawn(async move {
            let executor = task_executor::TaskExecutor {
                pool: &pool,
                channel: &channel,
                settings: &settings,
                workdir_root: app_local_data_dir,
                user_agent: crate::USER_AGENT.to_string(),
            };

            loop {
                if let Err(err) = executor.execute_root_task(crate::CID).await {
                    if let bridge_common::errors::Error::Executor(
                        bridge_common::task_executor::Error::NoRootTasks,
                    ) = err
                    {
                        trace!("No root tasks to execute, waiting...");

                        sleep(Duration::from_secs(1)).await;
                    } else {
                        error!("Failed to execute task: {:?}", err);
                    }
                }
            }
        });

        debug!("-- Thread #{} started", i);
    }
}
