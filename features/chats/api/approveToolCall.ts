// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const approveToolCall = (messageId: number) => {
  return invoke('approve_tool_call', { messageId })
}
