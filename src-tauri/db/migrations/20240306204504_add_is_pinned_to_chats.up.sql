-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

ALTER TABLE chats ADD COLUMN is_pinned BOOLEAN NOT NULL DEFAULT false;
