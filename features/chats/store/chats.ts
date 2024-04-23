// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0
import { listen } from '@tauri-apps/api/event'

import { type Chat } from '~/entities/chat'
import type { BridgeEvent } from '~/entities/events'
import { useToast } from '~/shared/lib'
import {
  createChat as createChatReq,
  deleteChat as deleteChatReq,
  listChats as listChatsReq,
  toggleIsPinned as toggleIsPinnedReq,
  updateChatModelFullName as updateChatModelFullNameReq,
} from '../api'
import { type CreateChat } from '../model'

export const useChatsStore = defineStore('chats', () => {
  const chats = ref<Chat[]>([])
  const pinnedChats = ref<Chat[]>([])
  const getById = (id: number | string | undefined | null): Chat | null => {
    if (id === undefined || id === null) {
      return null
    }
    if (typeof id === 'string') {
      id = parseInt(id, 10)
    }

    return chats.value.find((a) => a.id === id) || pinnedChats.value.find((a) => a.id === id) || null
  }
  const listChats = async (isPinned?: boolean) => {
    const allChats = await listChatsReq(isPinned)
    pinnedChats.value = allChats.filter((chat) => chat.is_pinned)
    chats.value = allChats.filter((chat) => !chat.is_pinned)
  }

  const createChat = async (request: CreateChat) => {
    const { data } = await createChatReq(request)
    const chat = data.value
    if (chat) {
      chat.agents_ids = [request.agent_id]
      chats.value.unshift(chat)
    }
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

  const toggleIsPinned = async (id: number) => {
    await toggleIsPinnedReq(id)
    listChats()
  }

  const updateChatModelFullName = async (id: number, modelFullName: Nullable<string>) => {
    await updateChatModelFullNameReq(id, modelFullName)
    const index = chats.value.findIndex((a) => a.id === id)
    if (index !== undefined && index !== -1) {
      chats.value[index].model_full_name = modelFullName
    }
  }

  const chatsUpdatedUnlisten = listen<BridgeEvent<Chat>>('chats:updated', (event) => {
    const chat = event.payload.data
    updateChat(chat)
  }).catch((error) => {
    useToast().errorToast(String(error))
  })

  const $reset = async () => {
    chats.value = []
    await chatsUpdatedUnlisten
  }

  return {
    $reset,
    chats,
    pinnedChats,
    createChat,
    deleteChat,
    getById,
    listChats,
    toggleIsPinned,
    updateChatModelFullName,
  }
})
