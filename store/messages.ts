// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/tauri'

export enum Status {
  WRITING = 'Writing',
  WAITING_FOR_TOOL_CALL = 'WaitingForToolCall',
  COMPLETED = 'Completed'
}

export enum Role {
  SYSTEM = 'System',
  USER = 'User',
  ASSISTANT = 'Assistant',
  TOOL = 'Tool'
}

export interface Message {
  id: number;
  chat_id: number;
  agent_id: number | null;
  status: Status;
  role: Role;
  content: string;
  prompt_tokens: number | null;
  completion_tokens: number | null;
  tool_calls: string | null;
  tool_call_id: string | null;
  created_at: Date;
}

export interface MessagesList {
  messages: Message[];
}

export interface CreateMessage {
  chat_id: number;
  text: string;
}

export interface DeleteMessage {
  id: number;
}

export interface ListMessages {
  chat_id: number;
}

export const useMessagesStore = defineStore('messages', {
  state: () => ({
    messages: [] as Message[]
  }),

  getters: {
    listByChatId: state => (id: number | string | undefined): Message[] => {
      if (id === undefined) {
        return []
      }

      if (typeof id === 'string') {
        id = parseInt(id, 10)
      }

      return state.messages.filter(a => a.chat_id === id)
    }
  },

  actions: {
    async listMessages(request: ListMessages) {
      const messages = await invoke<MessagesList>('list_messages', { request })
      this.messages = messages.messages
    },

    createMessage(request: CreateMessage) {
      return invoke<Message>('create_message', { request })
    },

    approveToolCall(messageId: number) {
      return invoke('approve_tool_call', { messageId })
    },

    async deleteMessage(request: DeleteMessage) {
      await invoke('delete_message', { request })
      const index = this.messages.findIndex(a => a.id === request.id)
      if (index !== undefined && index !== -1) {
        this.messages.splice(index, 1)
      }
    }
  }
})
