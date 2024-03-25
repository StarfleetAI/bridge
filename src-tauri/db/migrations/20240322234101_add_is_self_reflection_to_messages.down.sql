-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

ALTER TABLE messages DROP COLUMN is_self_reflection;
ALTER TABLE messages DROP COLUMN is_internal_tool_output;
