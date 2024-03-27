-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

ALTER TABLE agents ADD COLUMN is_code_interpreter_enabled BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE agents ADD COLUMN is_web_browser_enabled BOOLEAN NOT NULL DEFAULT false;
