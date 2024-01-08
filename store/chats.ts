// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/tauri'

export interface Chat {
  id: number;
  title: string;
  created_at: Date;
  updated_at: Date;
}

export interface ChatsList {
  chats: Chat[];
}

export interface CreateChat {
  agent_id: number;
}

export interface DeleteChat {
  id: number;
}

export interface GetChat {
  id: number;
}

export const useChatsStore = defineStore('chats', {
  state: () => ({
    chats: [] as Chat[]
  }),

  getters: {
    getById: state => (id: number | string | undefined): Chat | undefined => {
      if (id === undefined) {
        return undefined
      }

      if (typeof id === 'string') {
        id = parseInt(id, 10)
      }

      return state.chats.find(a => a.id === id)
    }
  },

  actions: {
    async listChats() {
      const chats = await invoke<ChatsList>('list_chats')
      this.chats = chats.chats
    },

    getChat(request: GetChat) {
      return new Promise((resolve, reject) => {
        try {
          invoke<Chat>('get_chat', { request }).then((chat) => {
            this.chats.push(chat)
            resolve(chat)
          })
        } catch (e) {
          reject(e)
        }
      })
    },

    createChat(request: CreateChat): Promise<Chat> {
      return new Promise((resolve, reject) => {
        try {
          invoke<Chat>('create_chat', { request }).then((chat) => {
            this.chats.push(chat)
            resolve(chat)
          })
        } catch (e) {
          reject(e)
        }
      })
    },

    async deleteChat(request: DeleteChat) {
      await invoke('delete_chat', { request })
      const index = this.chats.findIndex(a => a.id === request.id)
      if (index !== undefined && index !== -1) {
        this.chats.splice(index, 1)
      }
    }
  }
})
