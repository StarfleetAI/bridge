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
