-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

CREATE TABLE IF NOT EXISTS abilities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    code TEXT NOT NULL DEFAULT '',
    created_at datetime NOT NULL,
    updated_at datetime NOT NULL
);
