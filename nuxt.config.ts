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

  modules: ['@nuxtjs/tailwindcss', '@pinia/nuxt', 'nuxt-typed-router', '@vueuse/nuxt', 'dayjs-nuxt'],

  routeRules: {
    '/': { redirect: '/agents' }
  },

  ssr: false,

  devtools: { enabled: false },

  typescript: {
    strict: true
  },

  vite: {
    css: {
      preprocessorOptions: {
        scss: {
          additionalData: `
            @import "@/assets/scss/default.scss";
            @import "@/assets/scss/theming.scss";
          `
        }
      }
    }
  }
})
