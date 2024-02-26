// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.vueApp.config.errorHandler = (err) => {
    console.error('Nuxt Error', err)
  }

  nuxtApp.hook('app:error', (error) => {
    console.error('App starting error', error)
    // handle error, e.g. report to a service
  })
})
