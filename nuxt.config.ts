// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export default defineNuxtConfig({
  app: {
    head: {
      bodyAttrs: {
        class: ['bg-gray-900', 'text-slate-50', 'select-none', 'cursor-default']
      }
    }
  },

  modules: [
    '@nuxtjs/tailwindcss',
    '@pinia/nuxt'
  ],

  routeRules: {
    '/': { redirect: '/agents' }
  },

  ssr: false,

  devtools: { enabled: false }
})
