// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { AbilitiesList } from '../model'

export const listAbilities = async () => {
  const { abilities } = await invoke<AbilitiesList>('list_abilities')

  return abilities
}
