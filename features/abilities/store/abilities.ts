// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0
import { type Ability } from '~/entities/abilities'
import {
  listAbilities as listAbilitiesReq,
  createAbility as createAbilityReq,
  updateAbility as updateAbilityReq,
  deleteAbility as deleteAbilityReq,
} from '../api'
import { type CreateAbility, type UpdateAbility } from '../model'

export const useAbilitiesStore = defineStore('abilities', () => {
  const abilities = ref<Ability[]>([])

  const getById = (id: number | string): Ability => {
    if (typeof id === 'string') {
      id = parseInt(id, 10)
    }
    return abilities.value.find((a) => a.id === id) as Ability
  }

  const listAbilities = async () => {
    const list = await listAbilitiesReq()
    list.forEach((a) => {
      abilities.value.push(a)
    })
  }

  const createAbility = async (request: CreateAbility) => {
    const created = await createAbilityReq(request)
    abilities.value.push(created)
  }

  const updateAbility = async (request: UpdateAbility) => {
    const updated = await updateAbilityReq(request)
    const index = abilities.value.findIndex((a) => a.id === updated.id)
    if (index !== undefined && index !== -1) {
      abilities.value.splice(index, 1, updated)
    }
  }

  const deleteAbility = async (id: number) => {
    await deleteAbilityReq(id)
    const index = abilities.value.findIndex((a) => a.id === id)
    if (index !== undefined && index !== -1) {
      abilities.value.splice(index, 1)
    }
  }

  return {
    abilities,
    getById,
    listAbilities,
    createAbility,
    updateAbility,
    deleteAbility,
  }
})
