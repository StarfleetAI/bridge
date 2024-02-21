// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Ability } from '~/entities/abilities'
import { type CreateAbility } from '../model'

export const createAbility = async (request: CreateAbility) => {
  return invoke<Ability>('create_ability', { request })
}
