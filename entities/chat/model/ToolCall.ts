// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export interface ToolCall {
  id: number
  function: {
    name: string
    arguments: string
  }
  type: string
}
