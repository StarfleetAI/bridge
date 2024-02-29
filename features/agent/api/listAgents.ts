// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type AgentsList } from '../model'

export const listAgents = async () => {
  return invoke<AgentsList>('list_agents')
}
