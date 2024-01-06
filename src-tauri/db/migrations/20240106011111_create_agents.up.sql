-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

CREATE TABLE IF NOT EXISTS agents (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    system_message TEXT NOT NULL DEFAULT '',
    created_at datetime NOT NULL,
    updated_at datetime NOT NULL
);

CREATE TABLE IF NOT EXISTS agent_abilities (
    agent_id INTEGER NOT NULL,
    ability_id INTEGER NOT NULL,
    FOREIGN KEY (agent_id) REFERENCES agents(id),
    FOREIGN KEY (ability_id) REFERENCES abilities(id)
);

CREATE INDEX IF NOT EXISTS agent_abilities_agent_id_idx ON agent_abilities (agent_id);
CREATE INDEX IF NOT EXISTS agent_abilities_ability_id_idx ON agent_abilities (ability_id);
