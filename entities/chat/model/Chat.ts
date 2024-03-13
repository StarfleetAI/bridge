// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export interface Chat {
  id: number
  title: string
  created_at: Date
  updated_at: Date
  is_pinned: boolean
  agents_ids: number[]
  model_full_name: Nullable<string>
}
