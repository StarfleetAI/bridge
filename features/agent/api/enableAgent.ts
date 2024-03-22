// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Agent } from '~/entities/agents'

export const enableAgent = (id: number, isEnabled: boolean) => {
  return invoke<Agent>('update_agent_is_enabled', { id, isEnabled })
}
