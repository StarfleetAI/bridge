// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Role } from './Role'
import { type Status } from './Status'
import { type ToolCall } from './ToolCall'

export interface Message {
  id: number
  chat_id: number
  agent_id: Nullable<number>
  status: Status
  role: Role
  content: string
  prompt_tokens: Nullable<number>
  completion_tokens: Nullable<number>
  tool_calls: Nullable<ToolCall[]>
  tool_call_id: Nullable<string>
  created_at: string
  is_internal_tool_output: boolean
}
