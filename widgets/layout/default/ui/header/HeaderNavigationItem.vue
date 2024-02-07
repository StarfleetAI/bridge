<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  const props = defineProps<{ to: string }>()
  const route = useRoute()

  const isActiveItem = computed(() => {
    if (route.path === props.to) {
      return true
    }

    if (route.path.startsWith(props.to)) {
      const pathAfterTo = route.path.slice(props.to.length)
      return pathAfterTo.startsWith('/') && pathAfterTo.length > 1
    }

    return false
  })
  const handleClick = () => {
    navigateTo(props.to)
  }
</script>

<template>
  <div
    :to="to"
    :class="['header-item py-3 px-4', { active: isActiveItem }]"
    @click="handleClick()"
  >
    <slot name="icon" />
    <div class="header-item__name"><slot name="name" /></div>
  </div>
</template>

<style scoped lang="scss">
  .header-item {
    color: var(--text-tertiary);
    cursor: pointer;

    &.active {
      background: var(--surface-1);
      cursor: auto;
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
