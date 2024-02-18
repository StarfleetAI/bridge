// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Agent } from '~/entities/agent'
import { type UpdateAgent } from '../model'

export const updateAgent = (request: UpdateAgent) => {
  return invoke<Agent>('update_agent', { request })
}
