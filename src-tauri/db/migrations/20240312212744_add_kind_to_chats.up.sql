-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

ALTER TABLE chats ADD COLUMN kind TEXT NOT NULL DEFAULT 'Direct';
