-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY,
    agent_id INTEGER NOT NULL,
    origin_chat_id INTEGER,
    control_chat_id INTEGER,
    execution_chat_id INTEGER,
    title TEXT NOT NULL DEFAULT '',
    summary TEXT NOT NULL DEFAULT '',
    status TEXT NOT NULL,
    ancestry TEXT,
    ancestry_level INTEGER NOT NULL DEFAULT 0,
    created_at datetime NOT NULL,
    updated_at datetime NOT NULL,

    FOREIGN KEY (agent_id) REFERENCES agents (id),
    FOREIGN KEY (origin_chat_id) REFERENCES chats (id),
    FOREIGN KEY (control_chat_id) REFERENCES chats (id),
    FOREIGN KEY (execution_chat_id) REFERENCES chats (id)
);

CREATE INDEX IF NOT EXISTS tasks_agent_id_idx ON tasks (agent_id);
CREATE INDEX IF NOT EXISTS tasks_origin_chat_id_idx ON tasks (origin_chat_id);
CREATE INDEX IF NOT EXISTS tasks_control_chat_id_idx ON tasks (control_chat_id);
CREATE INDEX IF NOT EXISTS tasks_execution_chat_id_idx ON tasks (execution_chat_id);
CREATE INDEX IF NOT EXISTS tasks_ancestry_idx ON tasks (ancestry, ancestry_level);
CREATE INDEX IF NOT EXISTS tasks_status_idx ON tasks (status);
CREATE INDEX IF NOT EXISTS tasks_created_at_idx ON tasks (created_at);
