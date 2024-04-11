-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

CREATE TABLE IF NOT EXISTS pages (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    text TEXT NOT NULL,
    created_at datetime NOT NULL,
    updated_at datetime NOT NULL
);

CREATE INDEX IF NOT EXISTS pages_title_idx ON pages (title);
