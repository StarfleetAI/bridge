-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

ALTER TABLE chats ADD COLUMN model_full_name TEXT NOT NULL DEFAULT 'OpenAI/gpt-3.5-turbo'
