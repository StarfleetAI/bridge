// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { UpdateSettings } from '../model'

export const updateSettings = (newSettings: UpdateSettings) => {
  return invoke('update_settings', { newSettings })
}
