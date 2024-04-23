// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::used_underscore_binding)]

use anyhow::Context;
use bridge_common::repo;
use bridge_common::repo::pages::CreateParams;
use bridge_common::repo::pages::UpdateParams;
use bridge_common::types::pages::Page;
use bridge_common::types::pages::ShortPage;
use chrono::DateTime;
use chrono::Utc;
use markdown::to_html;
use serde::{Deserialize, Serialize};
use tauri::State;
use tracing::debug;
use tracing::instrument;

use crate::types::{DbPool, Result};

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PagesListResponse {
    pages: Vec<ShortPage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePageRequest {
    title: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePageRequest {
    id: i32,
    title: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageResponse {
    id: i32,
    title: String,
    text_markdown: String,
    text_html: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<Page> for PageResponse {
    fn from(page: Page) -> Self {
        Self {
            id: page.id,
            title: page.title,
            text_html: to_html(&page.text),
            text_markdown: page.text,
            created_at: page.created_at,
            updated_at: page.updated_at,
        }
    }
}

/// List all pages.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
#[tauri::command]
#[instrument(skip(pool))]
pub async fn list_pages(pool: State<'_, DbPool>) -> Result<Vec<ShortPage>> {
    debug!("Listing pages");

    let pages = repo::pages::list(&*pool, crate::CID).await?;

    Ok(pages)
}

/// Get raw page content by id.
///
/// # Errors
///
/// Returns error if page with given id does not exist.
#[tauri::command]
pub async fn get_page(id: i32, pool: State<'_, DbPool>) -> Result<PageResponse> {
    let page = repo::pages::get(&*pool, crate::CID, id)
        .await
        .with_context(|| "Failed to get page")?;

    Ok(page.into())
}

/// Create new page.
///
/// # Errors
///
/// Returns error if there was a problem while creating new page.
#[tauri::command]
#[instrument(skip(pool))]
pub async fn create_page(
    request: CreatePageRequest,
    pool: State<'_, DbPool>,
) -> Result<PageResponse> {
    debug!("Creating page");

    let page = repo::pages::create(
        &*pool,
        crate::CID,
        CreateParams {
            title: request.title,
            text: request.text,
        },
    )
    .await?;

    Ok(page.into())
}

/// Update page content by id.
///
/// # Errors
///
/// Returns error if there was a problem while updating page content.
#[instrument(skip(pool))]
#[tauri::command]
pub async fn update_page(
    request: UpdatePageRequest,
    pool: State<'_, DbPool>,
) -> Result<PageResponse> {
    debug!("Updating page");

    let updated_page = repo::pages::update(
        &*pool,
        crate::CID,
        request.id,
        UpdateParams {
            title: request.title,
            text: request.text,
        },
    )
    .await?;

    Ok(updated_page.into())
}

/// Delete page by id.
///
/// # Errors
///
/// Returns error if there was a problem while deleting page.
#[instrument(skip(pool))]
#[tauri::command]
pub async fn delete_page(id: i32, pool: State<'_, DbPool>) -> Result<()> {
    debug!("Deleting page");

    repo::pages::delete(&*pool, crate::CID, id).await?;

    Ok(())
}
