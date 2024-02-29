// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export interface Agent {
  id: number
  name: string
  description: string
  system_message: string
  ability_ids: number[]
  created_at: Date
  updated_at: Date
}
