// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0
import type { UnlistenFn } from '@tauri-apps/api/event'
import { listen } from '@tauri-apps/api/event'
import { useChatsStore } from '~/features/chats'
import { type Message } from '~/entities/chat'
import { BRIDGE_AGENT_ID } from '~/shared/lib'
import { listChatMessages, createMessage as createMessageReq, deleteMessage as deleteMessageReq } from '../api'

type ChatId = number

export const useMessagesStore = defineStore('messages', () => {
  const messages = ref<Record<ChatId, Message[]>>({})
  const getById = ({ id, chat_id }: Message) => {
    return messages.value[chat_id]?.find((a) => a.id === id)
  }

  const listMessages = async (chatId: number) => {
    messages.value[chatId] = await listChatMessages(chatId)
  }

  const createMessage = async (text: string, chat_id?: number) => {
    if (!chat_id) {
      const { createChat } = useChatsStore()
      const newChat = await createChat({ agent_id: BRIDGE_AGENT_ID })
      chat_id = newChat.id
      await navigateTo({ name: 'chats', query: { id: newChat.id } })
    }
    await createMessageReq({ text, chat_id })
  }

  const deleteMessage = async ({ id, chat_id }: Message) => {
    await deleteMessageReq(id)
    if (messages.value[chat_id]) {
      const index = messages.value[chat_id].findIndex((a) => a.id === id)
      if (index !== undefined && index !== -1) {
        messages.value[chat_id].splice(index, 1)
      }
    }
  }

  const addMessage = (message: Message) => {
    if (!messages.value[message.chat_id]) {
      messages.value[message.chat_id] = []
    }
    messages.value[message.chat_id].push(message)
  }

  const updateMessage = (message: Message) => {
    const index = messages.value[message.chat_id].findIndex((a) => a.id === message.id)
    if (index !== undefined && index !== -1) {
      messages.value[message.chat_id].splice(index, 1, message)
    } else {
      addMessage(message)
    }
  }

  const msgCreatedUnlisten: Promise<UnlistenFn> = listen('messages:created', (event) => {
    addMessage(event.payload as Message)
  })
  const msgUpdatedUnlisten: Promise<UnlistenFn> = listen('messages:updated', (event) => {
    const msg = event.payload as Message
    updateMessage(msg)
  })

  const $reset = async () => {
    messages.value = []
    await msgCreatedUnlisten
    await msgUpdatedUnlisten
  }

  return {
    messages,
    getById,
    listMessages,
    createMessage,
    deleteMessage,
    addMessage,
    updateMessage,
    $reset,
  }
})
