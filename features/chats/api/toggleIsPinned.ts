// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0
export const toggleIsPinned = (id: number) => {
  return invoke('toggle_chat_is_pinned', { id })
}
