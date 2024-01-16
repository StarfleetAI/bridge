// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/tauri'

export interface Ability {
  id: number;
  name: string;
  description: string;
  code: string;
  created_at: Date;
  updated_at: Date;
}

export interface AbilitiesList {
  abilities: Ability[];
}

export interface CreateAbility {
  name: string;
  description: string;
  code: string;
}

export interface UpdateAbility {
  id: number;
  name: string;
  description: string;
  code: string;
}

export const useAbilitiesStore = defineStore('abilities', {
  state: () => ({
    abilities: [] as Ability[]
  }),

  getters: {
    getById: state => (id: number | string | undefined): Ability | undefined => {
      if (id === undefined) {
        return undefined
      }

      if (typeof id === 'string') {
        id = parseInt(id, 10)
      }

      return state.abilities.find(a => a.id === id)
    }
  },

  actions: {
    async listAbilities() {
      const abilities = await invoke<AbilitiesList>('list_abilities')
      this.abilities = abilities.abilities
    },

    async createAbility(request: CreateAbility) {
      const created = await invoke<Ability>('create_ability', { request })
      this.abilities.push(created)
    },

    async updateAbility(request: UpdateAbility) {
      const updated = await invoke<Ability>('update_ability', { request })
      const index = this.abilities.findIndex(a => a.id === updated.id)
      if (index !== undefined && index !== -1) {
        this.abilities.splice(index, 1, updated)
      }
    },

    async deleteAbility(id: number) {
      await invoke('delete_ability', { id })
      const index = this.abilities.findIndex(a => a.id === id)
      if (index !== undefined && index !== -1) {
        this.abilities.splice(index, 1)
      }
    }
  }
})
