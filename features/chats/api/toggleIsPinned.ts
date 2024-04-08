// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0
export const toggleIsPinned = (id: number) => {
  return useInvoke({ cmd: 'toggle_chat_is_pinned', args: { id } })
}
