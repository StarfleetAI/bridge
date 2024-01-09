-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

ALTER TABLE abilities ADD COLUMN parameters_json TEXT NOT NULL DEFAULT '{}';
