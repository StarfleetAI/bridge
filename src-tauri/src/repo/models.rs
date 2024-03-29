// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Executor, Sqlite};
use tracing::{instrument, trace};

use crate::types::Result;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/";
const GROQ_API_URL: &str = "https://api.groq.com/v1/";

#[derive(
    Serialize, Deserialize, Debug, sqlx::Type, Default, PartialEq, Eq, Clone, Ord, PartialOrd,
)]
pub enum Provider {
    #[default]
    OpenAI,
    Groq,
}

impl From<String> for Provider {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Groq" => Provider::Groq,
            _ => Provider::OpenAI,
        }
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    // Provider of the model
    pub provider: Provider,
    // Name of the model (e.g. `gpt-4-turbo-preview`)
    pub name: String,
    // Context window size
    pub context_length: i64,
    // Maximum new tokens model can generate
    pub max_tokens: i64,
    // If model can take text input
    pub text_in: bool,
    // If model can generate text output
    pub text_out: bool,
    // If model can take image input
    pub image_in: bool,
    // If model can generate image output
    pub image_out: bool,
    // If model can take audio input
    pub audio_in: bool,
    // If model can generate audio output
    pub audio_out: bool,
    // If model has function calling capabilities
    pub function_calling: bool,
    // Base URL for the model's API. Leave empty to use provider's default
    pub api_url: Option<String>,
    // API key for the API. Leave empty to use provider's default
    pub api_key: Option<String>,
    // If model is managed by Bridge
    pub is_system: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Model {
    #[must_use]
    pub fn api_url_or_default(&self) -> &str {
        match self.api_url {
            Some(ref url) => url,
            None => match self.provider {
                Provider::OpenAI => OPENAI_API_URL,
                Provider::Groq => GROQ_API_URL,
            },
        }
    }
}

/// Get model by full name (`provider/name`).
///
/// # Errors
///
/// Returns error if there was a problem while fetching model.
#[instrument(skip(executor))]
pub async fn get<'a, E>(executor: E, full_name: &str) -> Result<Model>
where
    E: Executor<'a, Database = Sqlite>,
{
    trace!("Getting model");
    let (provider, name) = full_name.split_once('/').context("Invalid model name")?;

    Ok(query_as!(
        Model,
        "SELECT * FROM models WHERE provider = $1 AND name = $2",
        provider,
        name
    )
    .fetch_one(executor)
    .await
    .context("Failed to get model")?)
}

/// List models
///
/// # Errors
///
/// Returns error if there was a problem while fetching models.
#[instrument(skip(executor))]
pub async fn list<'a, E>(executor: E) -> Result<Vec<Model>>
where
    E: Executor<'a, Database = Sqlite>,
{
    Ok(query_as!(Model, "SELECT * FROM models")
        .fetch_all(executor)
        .await
        .context("Failed to list models")?)
}
