// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export interface PageState {
  scrollPosition: number
  formData?: Record<string, unknown>
}

export interface State {
  pageStates: Record<string, PageState>
}

const STACK_LENGTH = 50

export const usePageStateStore = defineStore('pageState', () => {
  const pageStatesStack = ref<Array<{ routePath: string; state: PageState }>>([])

  function saveState(routePath: string, state: PageState) {
    const existingStateIndex = pageStatesStack.value.findIndex((item) => item.routePath === routePath)
    if (existingStateIndex !== -1) {
      pageStatesStack.value.splice(existingStateIndex, 1)
    }
    pageStatesStack.value.push({ routePath, state })
    if (pageStatesStack.value.length > STACK_LENGTH) {
      pageStatesStack.value.shift()
    }
  }

  function getState(routePath: string): PageState | null {
    const foundItem = pageStatesStack.value.find((item) => item.routePath === routePath)
    return foundItem ? foundItem.state : null
  }

  function reset() {
    pageStatesStack.value = []
  }

  return {
    pageStatesStack,
    saveState,
    getState,
    reset,
  }
})
