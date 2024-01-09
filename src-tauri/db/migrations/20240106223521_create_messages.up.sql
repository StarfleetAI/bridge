-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

CREATE TABLE IF NOT EXISTS messages (
    id INTEGER PRIMARY KEY,
    chat_id INTEGER NOT NULL,
    agent_id INTEGER,
    status TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT,
    prompt_tokens INTEGER,
    completion_tokens INTEGER,
    tool_calls TEXT,
    tool_call_id TEXT,
    created_at datetime NOT NULL,
    FOREIGN KEY (chat_id) REFERENCES chats(id),
    FOREIGN KEY (agent_id) REFERENCES agents(id)
);

CREATE INDEX IF NOT EXISTS messages_chat_id_idx ON messages (chat_id);
CREATE INDEX IF NOT EXISTS messages_agent_id_idx ON messages (agent_id);
