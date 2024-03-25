-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

ALTER TABLE messages ADD COLUMN is_self_reflection BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE messages ADD COLUMN is_internal_tool_output BOOLEAN NOT NULL DEFAULT false;
