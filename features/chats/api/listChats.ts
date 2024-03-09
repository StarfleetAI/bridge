// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Chat } from '~/entities/chat'
import { type ChatsList } from '../model'
import { listChatsAgents } from './listChatsAgents'

export const listChats = async (isPinned?: boolean): Promise<Chat[]> => {
  const [chatsRes, chatsAgents] = await Promise.all([invoke<ChatsList>('list_chats', { isPinned }), listChatsAgents()])
  const { chats } = chatsRes
  chats.forEach((chat) => (chat.agents_ids = chatsAgents[chat.id] || []))
  return chats
}
