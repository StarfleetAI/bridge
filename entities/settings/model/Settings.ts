// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Provider } from '~/shared/model'

export interface Settings {
  api_keys: Record<Provider, string>
  python_path: string
  agents?: unknown
  default_model: Nullable<string>
}
