// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type TaskStatus } from './TaskStatus'

export interface Task {
  id: number
  agent_id: number
  origin_chat_id: undefined | number
  control_chat_id: undefined | number
  execution_chat_id: undefined | number
  title: string
  summary: string
  status: TaskStatus
  ancestry: undefined | string
  ancestry_level: number
  created_at: Date
  updated_at: Date
}
