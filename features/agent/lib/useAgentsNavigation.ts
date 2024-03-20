// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { useRouteQuery } from '@vueuse/router'

export const useAgentsNavigation = () => {
  const isCreateAgent = useRouteQuery('create', 'false', { transform: (value: 'false' | 'true') => value === 'true' })
  const isEditAgent = useRouteQuery('edit', 'false', { transform: (value: 'false' | 'true') => value === 'true' })
  const enableCreateAgent = () => {
    isCreateAgent.value = true
    isEditAgent.value = false
    selectedAgent.value = null
  }
  const enableEditAgent = () => {
    isCreateAgent.value = false
    isEditAgent.value = true
  }
  const disableCreateAgent = () => {
    isCreateAgent.value = false
    isEditAgent.value = false
    selectedAgent.value = null
  }

  const selectedAgent = useRouteQuery('agent', '', {
    transform: (value: string) => (isNaN(Number(value)) ? null : Number(value)),
  })
  const setSelectedAgent = (id: Nullable<number>) => {
    disableCreateAgent()
    selectedAgent.value = id
  }
  return {
    isCreateAgent: readonly(isCreateAgent),
    isEditAgent: readonly(isEditAgent),
    enableCreateAgent,
    enableEditAgent,
    disableCreateAgent,
    selectedAgent: readonly(selectedAgent),
    setSelectedAgent,
  }
}
