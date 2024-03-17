// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { usePageStateStore, type PageState } from '@/shared/lib/pageState'
import { simpleHash } from '@/shared/lib/simpleHash'

export default defineNuxtRouteMiddleware((to, from) => {
  const pageStateStore = usePageStateStore()

  const savedState = pageStateStore.getState(simpleHash(to.path))
  if (savedState) {
    window.scrollTo(0, savedState.scrollPosition)
  }

  if (from.path) {
    const currentState: PageState = {
      scrollPosition: window.scrollY,
    }
    pageStateStore.saveState(simpleHash(from.path), currentState)
  }
})
