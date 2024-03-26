// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const enum TaskResultKind {
  Text = 'Text',
  Url = 'Url',
}

export interface TaskResult {
  id: number
  agent_id: number
  task_id: number
  kind: TaskResultKind
  data: string
  parsed_data?: string
  created_at: string
  updated_at: string
}

export type TaskResults = TaskResult[]
