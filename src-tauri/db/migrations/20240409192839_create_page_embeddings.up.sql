-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

CREATE TABLE IF NOT EXISTS page_embeddings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    page_id INTEGER NOT NULL,
    text TEXT NOT NULL,
    embeddings BLOB NOT NULL,
    FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS page_embeddings_text_idx ON page_embeddings (text);
