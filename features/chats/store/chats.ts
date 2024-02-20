// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Chat } from '~/entities/chat'
import { listChats as listChatsReq, deleteChat as deleteChatReq, createChat as createChatReq } from '../api'
import { type CreateChat } from '../model'

export const useChatsStore = defineStore('chats', () => {
  const chats = ref<Chat[]>([])
  const getById = (id: number | string | undefined): Chat | undefined => {
    if (id === undefined) {
      return undefined
    }
    if (typeof id === 'string') {
      id = parseInt(id, 10)
    }
    return chats.value.find((a) => a.id === id)
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

  return {
    chats,
    getById,
    listChats,
    createChat,
    deleteChat,
  }
})
