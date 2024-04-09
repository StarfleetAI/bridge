// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { useRouteQuery } from '@vueuse/router'

/**
 * @param count The total number of items
 * @param pageSize The number of items per page
 */
export const usePagination = ({ count, pageSize }: { count: Ref<number>; pageSize: Ref<number> }) => {
  const currentPage = ref(1)
  const totalPages = computed(() => Math.ceil(count.value / pageSize.value))
  const setPage = (val: number) => {
    currentPage.value = val
    routeQueryPage.value = val
  }
  const routeQueryPage = useRouteQuery('page', undefined, {
    transform(val) {
      if (val) {
        return Number(val)
      }
      return undefined
    },
  })

  if (routeQueryPage.value) {
    currentPage.value = routeQueryPage.value
  }

  const resetPage = () => {
    currentPage.value = 1
  }

  return {
    currentPage,
    pageSize,
    totalPages,
    setPage,
    resetPage,
  }
}
