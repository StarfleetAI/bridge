// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { simpleHash } from './simpleHash'

export interface PageState {
  scrollPosition: number
  formData?: Record<string, unknown>
}

export interface State {
  pageStates: Record<string, PageState>
}

export const usePageStateStore = defineStore('pageState', {
  state: (): State => ({
    pageStates: {},
  }),
  actions: {
    saveState(routePath: string, state: PageState) {
      const hash = simpleHash(routePath)
      this.pageStates[hash] = state
    },
    getState(routePath: string): PageState | null {
      const hash = simpleHash(routePath)
      return this.pageStates[hash] || null
    },
  },
})
