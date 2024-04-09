// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export interface UpdateAgent {
  id: number
  name: string
  description: string
  system_message: string
  ability_ids: number[]
  is_code_interpreter_enabled: boolean
  is_web_browser_enabled: boolean
}
