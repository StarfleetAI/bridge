// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const denyToolCall = (messageId: number) => {
  return useInvoke({ cmd: 'deny_tool_call', args: { messageId } })
}
