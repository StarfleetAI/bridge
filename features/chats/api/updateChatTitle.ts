// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Chat } from '~/entities/chat'

export const updateChatTitle = async (request: { id: number; title: string }) => {
  return invoke<Chat>('update_chat_title', { ...request })
}
