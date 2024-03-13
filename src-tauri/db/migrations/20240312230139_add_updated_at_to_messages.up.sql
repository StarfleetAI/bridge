-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

ALTER TABLE messages ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;
