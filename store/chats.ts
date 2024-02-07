// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { invoke } from '@tauri-apps/api/tauri'
import { defineStore } from 'pinia'

export interface Chat {
  id: number
  title: string
  created_at: Date
  updated_at: Date
}

export interface ChatsList {
  chats: Chat[]
}

export interface CreateChat {
  agent_id: number
}

export const useChatsStore = defineStore('chats', {
  state: () => ({
    chats: [] as Chat[]
  }),

  getters: {
    getById:
      (state) =>
      (id: number | string | undefined): Chat | undefined => {
        if (id === undefined) {
          return undefined
        }

        if (typeof id === 'string') {
          id = parseInt(id, 10)
        }

        return state.chats.find((a) => a.id === id)
      }
  },

  actions: {
    async listChats() {
      const chats = await invoke<ChatsList>('list_chats')
      this.chats = chats.chats
    },

    async getChat(id: number) {
      const chat = await invoke<Chat>('get_chat', { id })
      this.chats.push(chat)
    },

    async createChat(request: CreateChat): Promise<Chat> {
      const chat = await invoke<Chat>('create_chat', { request })
      this.chats.push(chat)

      return chat
    },

    async deleteChat(id: number) {
      await invoke('delete_chat', { id })
      const index = this.chats.findIndex((a) => a.id === id)
      if (index !== undefined && index !== -1) {
        this.chats.splice(index, 1)
      }
    }
  }
})
