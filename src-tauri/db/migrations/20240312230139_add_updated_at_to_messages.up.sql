-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

ALTER TABLE messages ADD COLUMN updated_at datetime NOT NULL;

UPDATE messages SET updated_at = created_at;
