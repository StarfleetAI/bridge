// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { UnlistenFn } from '@tauri-apps/api/event'
import { listen } from '@tauri-apps/api/event'
import { useChatsStore, type EditMessage } from '~/features/chats'
import { type Message } from '~/entities/chat'
import {
  listChatMessages,
  createMessage as createMessageReq,
  deleteMessage as deleteMessageReq,
  editMessage as editMessageReq,
} from '../api'

type ChatId = number
interface CreateMessageParams {
  text: string
  agent_id: number
  chat_id?: number
  model_full_name: Nullable<string>
}
export const useMessagesStore = defineStore('messages', () => {
  const messages = ref<Record<ChatId, Message[]>>({})
  const getById = ({ id, chat_id }: Message) => {
    return messages.value[chat_id]?.find((message) => message.id === id)
  }

  const listMessages = async (chatId: number) => {
    messages.value[chatId] = await listChatMessages(chatId)
  }

  const createMessage = async ({ text, agent_id, chat_id, model_full_name }: CreateMessageParams) => {
    if (!chat_id) {
      const { createChat, updateChatModelFullName } = useChatsStore()
      const newChat = await createChat({ agent_id })
      await updateChatModelFullName(newChat.id, model_full_name)
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

  const editMessage = async (params: EditMessage, chat_id: number) => {
    const updatedMessage = await editMessageReq(params)
    if (messages.value[chat_id]) {
      const index = messages.value[chat_id].findIndex((a) => a.id === updatedMessage.id)
      if (index !== undefined && index !== -1) {
        messages.value[chat_id].splice(index, 1, updatedMessage)
      }
    }
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
    editMessage,
    $reset,
  }
})
