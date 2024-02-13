// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Pinia, type Store, getActivePinia } from 'pinia'

interface ExtendedPinia extends Pinia {
  _s: Map<string, Store>
}
export const useResetStores = () => {
  const pinia = getActivePinia() as ExtendedPinia

  if (!pinia) {
    throw new Error('There is no stores')
  }

  // eslint-disable-next-line no-underscore-dangle
  return pinia._s.forEach((store) => store.$reset())
}
