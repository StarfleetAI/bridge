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
    :class="['header-item', { active: isActiveItem }]"
    @click="handleClick()"
  >
    <div class="header-item__icon">
      <slot name="icon" />
    </div>
    <div class="header-item__name">
      <slot name="name" />
    </div>
  </div>
</template>

<style scoped lang="scss">
  .header-item {
    position: relative;
    width: 100%;
    height: 56px;
    color: var(--text-tertiary);

    &__icon {
      display: flex;
      flex-shrink: 0;
    }

    &:hover {
      color: var(--text-secondary);

      .header-item__name {
        display: block;
        color: var(--text-secondary);
      }
    }

    &.active {
      color: var(--button-primary);
    }

    @include flex(column, center, center);
  }

  .header-item__name {
    position: absolute;
    bottom: 0;
    display: none;

    @include font-inter-400(10px, 14px, var(--text-tertiary));
  }

  .header-item.active .header-item__name {
    color: var(--text-primary);
  }
</style>
