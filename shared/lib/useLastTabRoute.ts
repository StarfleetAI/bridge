// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

interface TabsLastRoutes {
  chats: string
  tasks: string
  agents: string
  documents: string
  settings: string
}

export type TabRoute = keyof TabsLastRoutes

export const useLastTabRoute = defineStore('lastTabRoute', () => {
  const tabsNavigationHistory = ref<TabsLastRoutes>({
    chats: '',
    tasks: '',
    agents: '',
    documents: '',
    settings: '',
  })

  const setTabLastRoute = (key: keyof TabsLastRoutes, value: string) => {
    tabsNavigationHistory.value[key] = value
  }

  const getTabLastRoute = (key: keyof TabsLastRoutes) => {
    return tabsNavigationHistory.value[key]
  }

  const $reset = () => {
    tabsNavigationHistory.value = {
      chats: '',
      tasks: '',
      agents: '',
      documents: '',
      settings: '',
    }
  }
  const resetTabLastRoute = (key: keyof TabsLastRoutes) => {
    tabsNavigationHistory.value[key] = ''
  }

  return {
    setTabLastRoute,
    getTabLastRoute,
    resetTabLastRoute,
    $reset,
  }
})
