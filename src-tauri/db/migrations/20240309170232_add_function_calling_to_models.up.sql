-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

ALTER TABLE models ADD COLUMN function_calling BOOLEAN NOT NULL DEFAULT FALSE;
