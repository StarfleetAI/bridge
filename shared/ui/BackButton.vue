<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import type { LocationQuery, RouteLocationRaw } from 'vue-router'
  import { useNavigationHistory } from '~/shared/utils'

  const props = defineProps<{
    routeName: string
  }>()

  const { findClosestRoute } = useNavigationHistory()
  const route = useRoute()
  const backRoute = computed(() => {
    const savedRoute = findClosestRoute(props.routeName)
    const result: { name: string; params?: unknown; query?: LocationQuery } = {
      name: props.routeName,
      params: route.params
    }
    if (savedRoute) {
      result.query = savedRoute.query
      result.params = savedRoute.params
    }
    return result as RouteLocationRaw
  })
</script>

<template>
  <NuxtLink
    :to="backRoute"
    class="back-button"
  >
    <slot />
  </NuxtLink>
</template>

<style lang="scss" scoped>
  .back-button {
    gap: 4px;
    cursor: pointer;

    @include flex(row, flex-start, center);
    @include font-inter-600(14px, 20px, var(--interactive-primary));
  }
</style>
~/shared/utils
