// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Chat } from '~/entities/chat'
import { type ChatsList } from '../model'
import { listChatsAgents } from './listChatsAgents'

export const listChats = async (isPinned?: boolean): Promise<Chat[]> => {
  const [chatsRes, chatsAgents] = await Promise.all([
    useInvoke<ChatsList>({ cmd: 'list_chats', args: { isPinned } }),
    listChatsAgents(),
  ])
  const { data } = chatsRes
  const { data: agents } = chatsAgents
  data.value?.chats.forEach((chat) => {
    chat.agents_ids = agents.value ? agents.value[chat.id] : []
  })
  return data.value?.chats || []
}
