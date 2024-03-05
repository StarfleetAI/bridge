-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

CREATE TABLE models (
    provider TEXT NOT NULL DEFAULT '',
    name TEXT NOT NULL DEFAULT '',
    context_length INTEGER NOT NULL,
    max_tokens INTEGER NOT NULL,
    text_in BOOLEAN NOT NULL DEFAULT 0,
    text_out BOOLEAN NOT NULL DEFAULT 0,
    image_in BOOLEAN NOT NULL DEFAULT 0,
    image_out BOOLEAN NOT NULL DEFAULT 0,
    audio_in BOOLEAN NOT NULL DEFAULT 0,
    audio_out BOOLEAN NOT NULL DEFAULT 0,
    api_url TEXT,
    api_key TEXT,
    is_system BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT '',
    updated_at DATETIME NOT NULL DEFAULT '',

    PRIMARY KEY (provider, name)
);

CREATE INDEX index_models_on_is_system ON models (is_system);

