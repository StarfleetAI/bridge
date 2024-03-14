// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { vClosePopper } from 'floating-vue'

export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.vueApp.directive('close-popper', vClosePopper)
})
