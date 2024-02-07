<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { defineProps, inject } from 'vue'

  const props = defineProps({
    tab: {
      required: true,
      type: [String, Number]
    }
  })

  const setActiveTab = inject<Function>('setActiveTab') as (tab: number | string) => void
  const activeTab = inject<Ref<number | string>>('activeTab')

  const isSelected = computed(() => activeTab?.value === props.tab)

  const selectTab = () => {
    setActiveTab(props.tab)
  }
</script>

<template>
  <div
    class="tab-item"
    :class="{ 'is-active': isSelected }"
    @click="selectTab"
  >
    <slot />
  </div>
</template>

<style lang="scss">
  .tab-items {
    position: relative;
    z-index: 1;
    padding: 12px 24px 0;

    @include flex(row, space-between, center);
  }

  .tab-item {
    flex: 1;
    height: 69px;
    padding: 12px 24px;
    border-radius: 20px;
    background: var(--surface-2);
    color: var(--text-primary);
    cursor: pointer;

    &__title {
      padding-left: 12px;

      @include font-inter-500(14px, 17px, var(--text-tertiary));
    }

    &.is-active {
      position: relative;
      border-bottom-right-radius: 0;
      border-bottom-left-radius: 0;
      background: #fff;
      box-shadow:
        0 2px 4px -1px rgba(0, 0, 0, 0.06),
        0 4px 6px -1px rgba(0, 0, 0, 0.1);

      .tab-item__title {
        padding-left: 0;
        color: var(--text-secondary);
      }

      .doughnut-chart {
        display: none;
      }
    }

    &__value {
      @include flex(row, flex-start, center);
    }

    @include flex(row, flex-start, center);
  }
</style>
