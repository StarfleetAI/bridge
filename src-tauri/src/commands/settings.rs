#![allow(clippy::used_underscore_binding)]

use crate::settings::Settings;
use crate::types::Result;

use tauri::{AppHandle, State};
use tokio::sync::RwLock;

/// Get the current settings.
///
/// # Errors
///
/// Does not return an error.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn get_settings(settings: State<'_, RwLock<Settings>>) -> Result<Settings> {
    Ok(settings.read().await.clone())
}

/// Update the settings.
///
/// # Errors
///
/// Will return an error if the settings can't be saved to disk.
///
/// # Panics
///
/// Will panic if the app local data dir can't be resolved.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn update_settings(
    settings: State<'_, RwLock<Settings>>,
    new_settings: Settings,
    app_handle: AppHandle,
) -> Result<()> {
    let mut st = settings.write().await;
    *st = new_settings;

    let app_local_data_dir = app_handle
        .path_resolver()
        .app_local_data_dir()
        .expect("Failed to get app local data dir")
        .to_str()
        .expect("Failed to convert app local data dir to string")
        .to_string();

    st.save_to_disk(&app_local_data_dir).await?;

    Ok(())
}
