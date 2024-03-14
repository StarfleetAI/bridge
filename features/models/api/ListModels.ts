// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0
import { type Model } from '~/entities/models'
export const listModels = (): Promise<Model[]> => {
  return invoke<Model[]>('list_models')
}
