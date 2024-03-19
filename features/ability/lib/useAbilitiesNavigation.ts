// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { useRouteQuery } from '@vueuse/router'

export const useAbilitiesNavigation = () => {
  const isCreateAbility = useRouteQuery('create', 'false', { transform: (value: 'false' | 'true') => value === 'true' })
  const isEditAbility = useRouteQuery('edit', 'false', { transform: (value: 'false' | 'true') => value === 'true' })
  const enableCreateAbility = () => {
    isCreateAbility.value = true
    isEditAbility.value = false
    selectedAbility.value = null
  }

  const enableEditAbility = () => {
    isCreateAbility.value = false
    isEditAbility.value = true
  }
  const disableCreateAbility = () => {
    isCreateAbility.value = false
    isEditAbility.value = false
    selectedAbility.value = null
  }

  const selectedAbility = useRouteQuery('ability', '', {
    transform: (value: string) => (isNaN(Number(value)) ? null : Number(value)),
  })
  const setSelectedAbility = (id: Nullable<number>) => {
    disableCreateAbility()
    selectedAbility.value = id
  }

  const enableCreateAbilityState = computed(() => isCreateAbility.value)

  return {
    isCreateAbility: readonly(isCreateAbility),
    isEditAbility: readonly(isEditAbility),
    enableCreateAbility,
    disableCreateAbility,
    selectedAbility: readonly(selectedAbility),
    setSelectedAbility,
    enableCreateAbilityState,
    enableEditAbility,
  }
}
