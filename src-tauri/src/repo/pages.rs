// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use markdown::to_html;
use serde::{Deserialize, Serialize, Serializer};

use sqlx::{query, query_as, Executor, Sqlite};


use crate::types::Result;

/// Safely render markdown in a page as an untrusted user input.
fn serialize_text<S>(text: &Option<String>, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&to_html(text.as_ref().unwrap_or(&String::new())))
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Page {
    pub id: i64,
    pub title: String,
    #[serde(serialize_with = "serialize_text")]
    pub text: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreatePageDTO {
    pub title: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ListPageDTO {
    pub id: i64,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Default)]
pub struct UpdatePageDTO {
    pub id: i64,
    pub title: Option<String>,
    pub text: Option<String>,
}

/// Create page.
///
/// # Errors
///
/// Returns error if there was a problem while creating page.
pub async fn create<'a, E>(executor: E, params: CreatePageDTO) -> Result<Page>
where
    E: Executor<'a, Database = Sqlite>,
{
    let current_datetime = Utc::now();
    let page = query_as!(
        Page,
        r#"
        INSERT INTO pages (title, text, created_at)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        params.title,
        params.text,
        current_datetime,
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
pub async fn list<'a, E>(executor: E) -> Result<Vec<ListPageDTO>>
where
    E: Executor<'a, Database = Sqlite>,
{
    let pages = query_as!(
        ListPageDTO,
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
pub async fn update_page<'a, E>(
    executor: E,
    id: i64,
    _title: Option<String>,
    text: Option<String>,
) -> Result<Page>
where
    E: Executor<'a, Database = Sqlite>,
{
    // let current_datetime = Utc::now();
    // let mut query = QueryBuilder::new("UPDATE pages SET ");
    //
    // if let Some(title) = title {
    //     query.push(" title = ");
    //     query.push_bind(title);
    // }
    //
    // if let Some(text) = text {
    //     if title {
    //
    //     }
    //     query.push(" text = ");
    //     query.push_bind(text);
    // }
    //
    // query.push(", updated_at = ");
    // query.push_bind(id);
    //
    // query.push(" WHERE id = ");
    // query.push_bind(id);
    //
    // query.push(" RETURNING *");
    //
    // query.build().sql().into()

    let current_datetime = Utc::now();
    let page = query_as!(
        Page,
        r#"
        UPDATE pages
        SET text = $2, updated_at = $3
        WHERE id = $1
        RETURNING id as "id!", title, text, created_at, updated_at
        "#,
        id,
        text,
        current_datetime
    )
    .fetch_one(executor)
    .await
    .with_context(|| "Failed to update page text")?;

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
