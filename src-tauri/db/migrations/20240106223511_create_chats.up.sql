-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

CREATE TABLE IF NOT EXISTS chats (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL DEFAULT '',
    created_at datetime NOT NULL,
    updated_at datetime NOT NULL
);

CREATE TABLE IF NOT EXISTS agents_chats (
    agent_id INTEGER NOT NULL,
    chat_id INTEGER NOT NULL,
    FOREIGN KEY (agent_id) REFERENCES agents(id),
    FOREIGN KEY (chat_id) REFERENCES chats(id)
);

CREATE INDEX IF NOT EXISTS agents_chats_agent_id_idx ON agents_chats (agent_id);
CREATE INDEX IF NOT EXISTS agents_chats_chat_id_idx ON agents_chats (chat_id);
