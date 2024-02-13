import { useRouteStore, useLastTabRoute, type TabRoute } from '~/shared/lib'

export default defineNuxtRouteMiddleware(async (to, from) => {
  if (process.env.VITEST_ENV) {
    return
  }

  if (to.path !== '/login') {
    const { savePreviousRoute } = useRouteStore()

    savePreviousRoute(from)
    const { setTabLastRoute } = useLastTabRoute()
    setTabLastRoute(from.name as TabRoute, from.fullPath)
  }
})
