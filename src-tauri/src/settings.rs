// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Default)]
pub struct Settings {
    pub database_url: Option<String>,
    pub openai_api_key: Option<String>,
    pub python_path: Option<String>,
}

impl Settings {
    #[must_use]
    pub fn new() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").ok(),
            openai_api_key: std::env::var("OPENAI_API_KEY").ok(),
            python_path: std::env::var("PYTHON_PATH").ok(),
        }
    }
}
