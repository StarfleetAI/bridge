// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use sqlx::{query, query_as, Executor, Sqlite};

use crate::types::Result;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Page {
    pub id: i64,
    pub title: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreatePageParams {
    pub title: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PageList {
    pub id: i64,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Create page.
///
/// # Errors
///
/// Returns error if there was a problem while creating page.
pub async fn create<'a, E>(executor: E, params: CreatePageParams) -> Result<Page>
where
    E: Executor<'a, Database = Sqlite>,
{
    let current_datetime = Utc::now();
    let page = query_as!(
        Page,
        r#"
        INSERT INTO pages (title, text, created_at, updated_at)
        VALUES ($1, $2, $3, $3)
        RETURNING *
        "#,
        params.title,
        params.text,
        current_datetime
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to create page")?;

    Ok(page)
}

/// List all pages.
///
/// # Errors
///
/// Returns error if there was a problem while accessing database.
pub async fn list<'a, E>(executor: E) -> Result<Vec<PageList>>
where
    E: Executor<'a, Database = Sqlite>,
{
    let pages = query_as!(
        PageList,
        r#"
        SELECT id, title, created_at, updated_at
        FROM pages
        ORDER BY id ASC
        "#,
    )
    .fetch_all(executor)
    .await
    .with_context(|| "Failed to list pages")?;

    Ok(pages)
}

/// Get page by id.
///
/// # Errors
///
/// Returns error if there was a problem while fetching page.
pub async fn get<'a, E>(executor: E, id: i64) -> Result<Page>
where
    E: Executor<'a, Database = Sqlite>,
{
    let page = query_as!(
        Page,
        r#"
        SELECT *
        FROM pages
        WHERE id = $1
        "#,
        id,
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to get page")?;

    Ok(page)
}

/// Update page text.
///
/// # Errors
///
/// Returns error if there was a problem while updating page text.
pub async fn update<'a, E>(executor: E, id: i64, data: CreatePageParams) -> Result<Page>
where
    E: Executor<'a, Database = Sqlite>,
{
    let current_datetime = Utc::now();

    let page = query_as!(
        Page,
        r#"
        UPDATE pages
        SET title = $2, text = $3, updated_at = $4
        WHERE id = $1
        RETURNING id as "id!", title, text, created_at, updated_at
        "#,
        id,
        data.title,
        data.text,
        current_datetime
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to update tool call id")?;

    Ok(page)
}

/// Delete page.
///
/// # Errors
///
/// Returns error if there was a problem while deleting page.
pub async fn delete<'a, E>(executor: E, id: i64) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    query!("DELETE FROM pages WHERE id = $1", id)
        .execute(executor)
        .await
        .with_context(|| "Failed to delete page")?;

    Ok(())
}
