// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Ability } from '~/entities/ability'
import { type UpdateAbility } from '../model'

export const updateAbility = (request: UpdateAbility) => {
  return invoke<Ability>('update_ability', { request })
}
