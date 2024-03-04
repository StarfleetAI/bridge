// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Agent } from '~/entities/agents'
import { DeleteIcon } from '~/shared/ui/icons'
import { useAgentsStore } from '../store'

export const useAgentActions = (agent: Ref<Agent>) => {
  const id = computed(() => agent.value.id)
  const { deleteAgent: deleteAgentReq } = useAgentsStore()

  const deleteAgent = computed(() => {
    return {
      label: 'Delete Agent',
      icon: DeleteIcon,
      action: () => deleteAgentReq(id.value),
    }
  })

  const baseActions = computed(() => {
    return [deleteAgent.value]
  })

  return computed(() => baseActions.value)
}
