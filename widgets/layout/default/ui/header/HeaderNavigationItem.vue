<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import type { TypedRouteLocationFromName } from '@typed-router/__router'
  import type { TabRoute } from '~/shared/lib'

  const props = defineProps<{ to: string | { name: TabRoute }; name: string }>()
  const route = useRoute()

  const isActiveItem = computed(() => {
    return route.name === props.name
  })
  const handleClick = () => {
    navigateTo(props.to as TypedRouteLocationFromName<TabRoute>)
  }
</script>

<template>
  <div
    :to="to"
    :class="['header-item py-3 px-4', { active: isActiveItem }]"
    @click="handleClick()"
  >
    <slot name="icon" />
  </div>
</template>

<style scoped lang="scss">
  .header-item {
    color: var(--text-tertiary);
    cursor: pointer;

    &.active {
      color: var(--button-primary);
    }

    @include flex($align-items: center, $gap: 8px);
  }

  .header-item__name {
    font-weight: 500;
    font-size: 14px;
    line-height: 20px;
  }

  .header-item.active .header-item__name {
    color: var(--text-primary);
  }
</style>
