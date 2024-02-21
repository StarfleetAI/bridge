// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type MessagesList } from '../model'

export const listChatMessages = async (chat_id: number) => {
  const { messages } = await invoke<MessagesList>('list_messages', { request: { chat_id } })
  return messages
}
