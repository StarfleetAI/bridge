// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { useRouteQuery } from '@vueuse/router'
export const useChatsNavigation = () => {
  // value is string with type 'chatId,agentId'
  const chatSettingsIsOpened = useRouteQuery('chat-settings', '', {
    transform: (value: string) => {
      return value === 'true'
    },
  })
  const setIsSettingsOpened = (value: boolean) => {
    chatSettingsIsOpened.value = value
  }
  const route = useRoute()
  const chatId = computed(() => (route.query.id ? Number(route.query.id) : undefined))
  return {
    chatSettingsIsOpened: readonly(chatSettingsIsOpened),
    chatId,
    setIsSettingsOpened,
  }
}
