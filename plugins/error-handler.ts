// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.vueApp.config.errorHandler = (err) => {
    console.warn(err)
  }

  nuxtApp.hook('app:error', (error) => {
    console.warn('App starting error', error)
    // handle error, e.g. report to a service
  })
})
