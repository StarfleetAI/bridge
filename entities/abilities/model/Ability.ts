// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export interface Ability {
  id: number
  name: string
  description: string
  code: string
  created_at: Date
  updated_at: Date
  parameters_json: string
}
