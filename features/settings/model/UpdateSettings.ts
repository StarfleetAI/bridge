// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Provider } from '~/shared/model'

export interface UpdateSettings {
  api_keys?: Record<Provider, string>
  agents?: unknown
  default_model: Nullable<string>
}
