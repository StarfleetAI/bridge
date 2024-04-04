// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};
use tokio::sync::RwLock;
use tracing::{debug, trace};
use tracing::{error, instrument};

use crate::abilities::{self};
use crate::chats::GetCompletionParams;
use crate::repo::models;
use crate::{chats, repo};
use crate::{
    clients::openai::{Client, CreateChatCompletionRequest},
    repo::pages::{Page, CreatePageDTO, UpdatePageDTO},
    settings::Settings,
    types::{DbPool, Result},
};
use crate::repo::pages::ListPageDTO;

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PagesList {
    pub pages: Vec<ListPageDTO>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePage {
    pub title: String,
    pub text: String,
}

/// List all pages.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
#[instrument(skip(pool))]
pub async fn list_pages(pool: State<'_, DbPool>) -> Result<Vec<ListPageDTO>> {
    debug!("Listing pages");

    let pages = repo::pages::list(&*pool).await?;

    Ok(pages)
}

/// Get raw page content by id.
///
/// # Errors
///
/// Returns error if page with given id does not exist.
#[tauri::command]
pub async fn get_raw_page_content(id: i64, pool: State<'_, DbPool>) -> Result<String> {
    let page = repo::pages::get(&*pool, id)
        .await
        .with_context(|| "Failed to get page")?;

    Ok(page.text.unwrap_or_default())
}

/// Create new page.
///
/// # Errors
///
/// Returns error if there was a problem while creating new page.
#[tauri::command]
#[instrument(skip(app_handle, pool, settings))]
pub async fn create_page(
    request: CreatePage,
    pool: State<'_, DbPool>,
) -> Result<()> {
    debug!("Creating page");

    let page = repo::pages::create(
        &*pool,
        CreatePageDTO {
            title: request.title,
            text: request.text,

            ..Default::default()
        },
    )
    .await?;

    Ok(())
}

/// Update page content by id.
///
/// # Errors
///
/// Returns error if there was a problem while updating page content.
#[instrument(skip(content, pool))]
#[tauri::command]
pub async fn update_page_content(
    id: i64,
    content: String,
    pool: State<'_, DbPool>,
) -> Result<Page> {
    debug!("Updating page content");

    let updated_page = repo::pages::update_page_text(&*pool, id, &content).await?;

    Ok(updated_page)
}

/// Delete page by id.
///
/// # Errors
///
/// Returns error if there was a problem while deleting page.
#[instrument(skip(pool))]
#[tauri::command]
pub async fn delete_page(id: i64, pool: State<'_, DbPool>) -> Result<()> {
    debug!("Deleting page");

    repo::pages::delete(&*pool, id).await
}
