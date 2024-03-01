// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Settings } from '~/entities/settings'

export const getSettings = () => {
  return invoke<Settings>('get_settings')
}
