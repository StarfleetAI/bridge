// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { useRouteQuery } from '@vueuse/router'

export const useAbilitiesNavigation = () => {
  const isCreateAbility = ref(false)
  const enableCreateAbility = () => {
    isCreateAbility.value = true
    selectedAbility.value = null
  }
  const disableCreateAbility = () => {
    isCreateAbility.value = false
  }

  const selectedAbility = useRouteQuery('ability', '', {
    transform: (value: string) => (isNaN(Number(value)) ? null : Number(value)),
  })
  const setSelectedAbility = (id: Nullable<number>) => {
    disableCreateAbility()
    selectedAbility.value = id
  }
  return {
    isCreateAbility: readonly(isCreateAbility),
    enableCreateAbility,
    disableCreateAbility,
    selectedAbility: readonly(selectedAbility),
    setSelectedAbility,
  }
}
