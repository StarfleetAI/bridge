-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

ALTER TABLE agents ADD COLUMN execution_steps_limit INTEGER DEFAULT NULL;
