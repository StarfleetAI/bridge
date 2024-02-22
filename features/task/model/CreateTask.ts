// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export interface CreateTask {
  agent_id: number
  title: string
  summary: string
  ancestry: undefined | string
}
