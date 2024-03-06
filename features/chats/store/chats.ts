// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0
import type { UnlistenFn } from '@tauri-apps/api/event'
import { listen } from '@tauri-apps/api/event'

import { type Chat } from '~/entities/chat'
import { listChats as listChatsReq, deleteChat as deleteChatReq, createChat as createChatReq } from '../api'
import { type CreateChat } from '../model'

export const useChatsStore = defineStore('chats', () => {
  const chats = ref<Chat[]>([])
  const getById = (id: number | string | undefined): Chat | null => {
    if (id === undefined) {
      return null
    }
    if (typeof id === 'string') {
      id = parseInt(id, 10)
    }
    return chats.value.find((a) => a.id === id) || null
  }
  const listChats = async () => {
    chats.value = await listChatsReq()
  }

  const createChat = async (request: CreateChat) => {
    const chat = await createChatReq(request)
    chats.value.unshift(chat)
    return chat
  }

  const deleteChat = async (id: number) => {
    await deleteChatReq(id)
    const index = chats.value.findIndex((a) => a.id === id)
    if (index !== undefined && index !== -1) {
      chats.value.splice(index, 1)
    }
  }
  const updateChat = (chat: Chat) => {
    const index = chats.value.findIndex((a) => a.id === chat.id)
    if (index !== undefined && index !== -1) {
      chats.value[index].title = chat.title
    }
  }
  const chatsUpdatedUnlisten: Promise<UnlistenFn> = listen('chats:updated', (event) => {
    const chat = event.payload as Chat
    updateChat(chat)
  })

  const $reset = async () => {
    chats.value = []
    await chatsUpdatedUnlisten
  }

  return {
    chats,
    getById,
    listChats,
    createChat,
    deleteChat,
    $reset,
  }
})
