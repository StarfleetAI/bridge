// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { useRouteQuery } from '@vueuse/router'
type ChatId = Nullable<number>
type AgentId = number
export const useChatsNavigation = () => {
  // value is string with type 'chatId,agentId'
  const chatSettings = useRouteQuery('chat-settings', '', {
    transform: (value: string | [ChatId, AgentId]) => {
      return value === '' ? null : (value as [ChatId, AgentId])
    },
  })
  const setIsSettingsOpened = (value: Nullable<[ChatId, AgentId]>) => {
    chatSettings.value = value
  }
  return {
    chatSettings: readonly(chatSettings),
    setIsSettingsOpened,
  }
}
