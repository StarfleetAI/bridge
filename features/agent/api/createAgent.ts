// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Agent } from '~/entities/agents'
import { type CreateAgent } from '../model'

export const createAgent = (request: CreateAgent) => {
  return invoke<Agent>('create_agent', { request })
}
