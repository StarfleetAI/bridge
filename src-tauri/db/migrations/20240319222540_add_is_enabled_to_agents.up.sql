-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

ALTER TABLE agents ADD COLUMN is_enabled BOOLEAN NOT NULL DEFAULT true;
