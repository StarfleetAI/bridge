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
    :class="['sidebar-item__container', { active: isActiveItem }]"
    @click="handleClick()"
  >
    <div class="sidebar-item__icon"><slot name="icon" /></div>
    <div class="sidebar-item__name"><slot name="name" /></div>
  </div>
</template>

<style scoped lang="scss">
  .sidebar-item {
    &__container {
      padding: 8px;
      border-radius: 8px;
      color: black;
      cursor: pointer;

      &.active {
        background: var(--surface-2);
        color: black;
        cursor: auto;
      }

      @include flex(row, flex-start, center);
    }

    &__name {
      @include font-inter-700(14px, 17px);
    }

    &__icon {
      margin-top: 2px;
      margin-right: 10px;
      color: black;
      transition: all 0.2s ease;
    }
  }
</style>
