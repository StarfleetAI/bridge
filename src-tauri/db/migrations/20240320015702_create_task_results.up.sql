-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

CREATE TABLE task_results (
    id INTEGER PRIMARY KEY,
    agent_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL,
    kind TEXT NOT NULL DEFAULT 'Text',
    data TEXT NOT NULL,
    created_at datetime NOT NULL,
    updated_at datetime NOT NULL,

    FOREIGN KEY (agent_id) REFERENCES agents (id),
    FOREIGN KEY (task_id) REFERENCES tasks (id)
);

CREATE INDEX task_results_agent_id_idx ON task_results (agent_id);
CREATE INDEX task_results_task_id_idx ON task_results (task_id);
