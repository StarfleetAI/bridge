// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { onMounted, onUnmounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { usePageStateStore, type PageState } from '@/shared/lib/pageState'

export const useSavePageState = () => {
  const pageStateStore = usePageStateStore()
  const route = useRoute()
  const savedState = ref<PageState>({ scrollPosition: 0 })

  onMounted(() => {
    const state = pageStateStore.getState(route.path)
    if (state) {
      savedState.value = state
      window.scrollTo(0, state.scrollPosition)
    }
  })

  onUnmounted(() => {
    pageStateStore.saveState(route.path, savedState.value)
  })

  return { savedState }
}
