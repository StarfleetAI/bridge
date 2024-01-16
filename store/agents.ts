// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/tauri'

export interface Agent {
  id: number;
  name: string;
  description: string;
  system_message: string;
  ability_ids: number[];
  created_at: Date;
  updated_at: Date;
}

export interface AgentsList {
  agents: Agent[];
}

export interface CreateAgent {
  name: string;
  description: string;
  system_message: string;
  ability_ids: number[];
}

export interface UpdateAgent {
  id: number;
  name: string;
  description: string;
  system_message: string;
  ability_ids: number[];
}

export const useAgentsStore = defineStore('agents', {
  state: () => ({
    agents: [] as Agent[]
  }),

  getters: {
    getById: state => (id: number | string | undefined): Agent | undefined => {
      if (id === undefined) {
        return undefined
      }

      if (typeof id === 'string') {
        id = parseInt(id, 10)
      }

      return state.agents.find(a => a.id === id)
    }
  },

  actions: {
    async listAgents() {
      const agents = await invoke<AgentsList>('list_agents')
      this.agents = agents.agents
    },

    async createAgent(request: CreateAgent) {
      const created = await invoke<Agent>('create_agent', { request })
      this.agents.push(created)
    },

    async updateAgent(request: UpdateAgent) {
      const updated = await invoke<Agent>('update_agent', { request })
      const index = this.agents.findIndex(a => a.id === updated.id)
      if (index !== undefined && index !== -1) {
        this.agents.splice(index, 1, updated)
      }
    },

    async deleteAgent(id: number) {
      await invoke('delete_agent', { id })
      const index = this.agents.findIndex(a => a.id === id)
      if (index !== undefined && index !== -1) {
        this.agents.splice(index, 1)
      }
    }
  }
})
