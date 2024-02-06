// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import * as Sentry from '@sentry/vue'
import type { Router } from 'vue-router'

export default defineNuxtPlugin((nuxtApp) => {
  const config = useRuntimeConfig()
  const router = nuxtApp.$router as Router
  nuxtApp.vueApp.config.errorHandler = (err) => {
    console.error('Vue Error', err)
  }

  Sentry.init({
    app: nuxtApp.vueApp,
    attachStacktrace: true,
    // debug: config.public.ENV !== 'production',
    dsn: config.public.sentryDsn as string,
    // release: config.public.SENTRY_RELEASE,
    enableTracing: true,
    enabled: process.env.NODE_ENV === 'production',
    // environment: config.public.SENTRY_ENVIRONMENT,
    integrations: [
      Sentry.browserTracingIntegration({
        router
      }),
      Sentry.replayIntegration()
    ],
    logErrors: true,
    replaysOnErrorSampleRate: 1,
    // Set tracesSampleRate to 1.0 to capture 100%
    // of transactions for performance monitoring.
    // plus for 100% of sessions with an error
    replaysSessionSampleRate: 0.1,

    // Capture Replay for 10% of all sessions,
    // We recommend adjusting this value in production
    tracesSampleRate: 0.2,
    trackComponents: true
  })

  return {
    provide: {
      sentryAddBreadcrumb: Sentry.addBreadcrumb,
      sentryCaptureException: Sentry.captureException,
      sentrySetContext: Sentry.setContext,
      sentrySetTag: Sentry.setTag,
      sentrySetUser: Sentry.setUser
    }
  }
})
