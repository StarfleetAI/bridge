// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { sentryVitePlugin } from '@sentry/vite-plugin'

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
    },
    plugins: [
      // Put the Sentry vite plugin after all other plugins
      sentryVitePlugin({
        authToken: process.env.SENTRY_AUTH_TOKEN,
        org: process.env.SENTRY_ORG_NAME,
        project: process.env.SENTRY_PROJECT_NAME,
        url: process.env.SENTRY_URL,
        sourcemaps: {
          assets: ['.nuxt/dist/**/*'],
          ignore: ['node_modules']
        },
        telemetry: false
      })
    ]
  }
})
