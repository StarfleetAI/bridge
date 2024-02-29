// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export interface UpdateAgent {
  id: number
  name: string
  description: string
  system_message: string
  ability_ids: number[]
}
