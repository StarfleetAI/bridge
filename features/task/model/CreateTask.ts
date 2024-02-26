// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { TaskStatus } from '~/entities/tasks'

export interface CreateTask {
  agent_id: number
  title: string
  summary: string
  ancestry?: string
  status: TaskStatus
}
