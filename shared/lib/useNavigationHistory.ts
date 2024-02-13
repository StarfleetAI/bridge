// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { RouteLocationNormalized } from 'vue-router'

const STACK_LENGTH = 20

export const useRouteStore = defineStore('navigationHistory', () => {
  const previousRoutesStack = ref<RouteLocationNormalized[]>([])

  function savePreviousRoute(route: RouteLocationNormalized) {
    previousRoutesStack.value.push(route)
    if (previousRoutesStack.value.length >= STACK_LENGTH) {
      previousRoutesStack.value.shift()
    }
  }
  const route = useRoute()

  const routeQueries = computed(() => route.query)
  const currentRouteName = computed(() => route.name)

  const findClosestRoute = (routeName: string) => {
    return previousRoutesStack.value.findLast((lastRoute) => lastRoute.name === routeName)
  }
  const $reset = () => {
    previousRoutesStack.value = []
  }
  return {
    $reset,
    currentRouteName,
    findClosestRoute,
    previousRoutesStack,
    routeQueries,
    savePreviousRoute
  }
})

export const useNavigationHistory = () => {
  const routeStore = useRouteStore()
  const { findClosestRoute, savePreviousRoute } = routeStore
  const { currentRouteName, previousRoutesStack, routeQueries } = storeToRefs(routeStore)

  return {
    currentRouteName,
    findClosestRoute,
    previousRoutesStack,
    routeQueries,
    savePreviousRoute
  }
}
