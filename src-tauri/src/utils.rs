// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::types::Result;

/// Emit event to main window.
///
/// # Errors
///
/// Returns error if there was a problem while emitting event.
pub fn emit_event<S>(app_handle: &AppHandle, event: &str, object: S) -> Result<()>
where
    S: Serialize + Clone,
{
    let window = app_handle
        .get_window("main")
        .context("Failed to get main window")?;

    window
        .emit_all(event, object)
        .context("Failed to emit event")?;

    Ok(())
}
