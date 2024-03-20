// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0
import { type Agent } from '~/entities/agents'
import {
  listAgents as listAgentsReq,
  deleteAgent as deleteAgentReq,
  createAgent as createAgentReq,
  updateAgent as updateAgentReq,
  enableAgent as enableAgentReq,
} from '../api'
import { type CreateAgent, type UpdateAgent } from '../model'

export const useAgentsStore = defineStore('agents', () => {
  const agents = ref<Agent[]>([])
  const getById = (id: Nullable<number | string>): Agent | undefined => {
    if (!id) {
      return undefined
    }
    if (typeof id === 'string') {
      id = parseInt(id, 10)
    }
    return agents.value.find((a) => a.id === id)
  }
  const listAgents = async () => {
    const agentsList = await listAgentsReq()
    agents.value = agentsList?.agents
  }

  const createAgent = async (request: CreateAgent) => {
    const created = await createAgentReq(request)
    agents.value.push(created)
  }

  const enableAgent = async (id: number, isEnabled: boolean) => {
    const updated = await enableAgentReq(id, isEnabled)
    const index = agents.value.findIndex((a) => a.id === updated.id)
    if (index !== undefined && index !== -1) {
      agents.value.splice(index, 1, updated)
    }
  }

  const updateAgent = async (request: UpdateAgent) => {
    const updated = await updateAgentReq(request)
    const index = agents.value.findIndex((a) => a.id === updated.id)
    if (index !== undefined && index !== -1) {
      agents.value.splice(index, 1, updated)
    }
  }

  const deleteAgent = async (id: number): Promise<void> => {
    await deleteAgentReq(id)
    const index = agents.value.findIndex((a) => a.id === id)
    if (index !== undefined && index !== -1) {
      agents.value.splice(index, 1)
    }
  }

  const $reset = () => {
    agents.value = []
  }

  return {
    agents,
    getById,
    listAgents,
    createAgent,
    enableAgent,
    updateAgent,
    deleteAgent,
    $reset,
  }
})
