// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Settings } from '~/entities/settings'
import { getSettings as getSettingsReq, updateSettings as updateSettingsReq } from '../api'
import type { UpdateSettings } from '../model'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<Nullable<Settings>>(null)
  const getSettings = async () => {
    settings.value = await getSettingsReq()
    if (settings.value?.default_model === null) {
      settings.value.default_model = 'OpenAI/gpt-3.5-turbo'
    }
  }
  const updateSettings = async (request: UpdateSettings) => {
    await updateSettingsReq(request)
    getSettings()
  }
  return {
    settings: readonly(settings),
    getSettings,
    updateSettings,
  }
})
