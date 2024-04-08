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

  modules: ['@pinia/nuxt', 'nuxt-typed-router', '@vueuse/nuxt', 'dayjs-nuxt'],

  routeRules: {
    '/': { redirect: '/chats' },
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
      {
        from: '~/shared/lib',
        imports: ['useInvoke'],
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
      include: [
        '@tauri-apps/api/event',
        'highlight.js',
        'highlightjs-copy',
        '@vueuse/integrations/useChangeCase',
        'prismjs',
        'prismjs/components/prism-python',
        'vue-prism-editor',
      ],
    },
  },
})
