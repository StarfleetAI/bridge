// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const useDevTools = () => {
  if (import.meta.env.DEV) {
    const config = useRuntimeConfig()
    // @ts-expect-error TODO extend window interface
    window.__VUE_DEVTOOLS_PORT__ = config.public.devtoolsPort
    useHead({
      script: [{ src: `http://localhost:${config.public.devtoolsPort}` }],
    })
  }
}
