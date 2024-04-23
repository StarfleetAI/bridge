// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { listen } from '@tauri-apps/api/event'
import { useChatsStore, type EditMessage } from '~/features/chats'
import { type Message } from '~/entities/chat'
import type { BridgeEvent } from '~/entities/events'
import { useToast } from '~/shared/lib'
import {
  createMessage as createMessageReq,
  deleteMessage as deleteMessageReq,
  editMessage as editMessageReq,
  listChatMessages as listChatMessagesReq,
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

  const listMessages = async (chat_id: number) => {
    const { data } = await listChatMessagesReq(chat_id)
    messages.value[chat_id] = data.value?.messages || []
  }

  const createMessage = async ({ text, agent_id, chat_id, model_full_name }: CreateMessageParams) => {
    if (!chat_id) {
      const { createChat, updateChatModelFullName } = useChatsStore()
      const newChat = await createChat({ agent_id })
      if (newChat) {
        await updateChatModelFullName(newChat.id, model_full_name)
        chat_id = newChat.id
        await navigateTo({
          name: 'chats',
          query: { id: newChat.id },
        })
      }
    }
    if (chat_id) {
      createMessageReq({
        text,
        chat_id,
      })
    }
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
    const { data } = await editMessageReq(params)
    if (messages.value[chat_id]) {
      const index = messages.value[chat_id].findIndex((a) => a.id === data.value?.id)
      if (index !== undefined && index !== -1 && data.value) {
        messages.value[chat_id].splice(index, 1, data.value)
      }
    }
  }

  const updateMessage = (message: Message) => {
    const index = messages.value[message.chat_id]?.findIndex((a) => a.id === message.id)
    if (index !== undefined && index !== -1) {
      messages.value[message.chat_id].splice(index, 1, message)
    } else {
      addMessage(message)
    }
  }

  const msgCreatedUnlisten = listen<BridgeEvent<Message>>('messages:created', (event) => {
    addMessage(event.payload.data)
  }).catch((e) => {
    useToast().errorToast(String(e))
  })
  const msgUpdatedUnlisten = listen<BridgeEvent<Message>>('messages:updated', (event) => {
    const msg = event.payload.data
    updateMessage(msg)
  }).catch((e) => {
    useToast().errorToast(String(e))
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
