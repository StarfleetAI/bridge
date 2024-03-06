// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export default defineNuxtConfig({
  app: {
    head: {
      bodyAttrs: {
        class: ['select-none', 'cursor-default'],
      },
    },
  },

  modules: ['@nuxtjs/tailwindcss', '@pinia/nuxt', 'nuxt-typed-router', '@vueuse/nuxt', 'dayjs-nuxt'],

  routeRules: {
    '/': { redirect: '/agents' },
  },

  runtimeConfig: {
    public: {
      devtoolsPort: '',
    },
  },

  ssr: false,

  devtools: { enabled: false },

  typescript: {
    strict: true,
  },
  dayjs: {
    defaultLocale: [
      'en',
      {
        weekStart: 1,
      },
    ],
  },

  imports: {
    presets: [
      {
        from: '@tauri-apps/api/tauri',
        imports: ['invoke'],
      },
    ],
  },

  css: ['~/assets/css/theming.css'],

  vite: {
    css: {
      preprocessorOptions: {
        scss: {
          additionalData: `
            @import "@/assets/scss/default.scss";
          `,
        },
      },
    },
    optimizeDeps: {
      include: ['@tauri-apps/api/event', 'highlight.js', 'highlightjs-copy'],
    },
  },
})
