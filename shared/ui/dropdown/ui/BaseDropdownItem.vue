<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  withDefaults(defineProps<{ iconPlacement?: 'start' | 'end' }>(), {
    iconPlacement: 'start',
  })
</script>

<template>
  <div :class="['dropdown-item', { reversed: iconPlacement === 'end' }]">
    <slot name="icon" />
    <slot name="label" />
  </div>
</template>

<style lang="scss" scoped>
  .dropdown-item {
    padding: 12px 16px;

    &:not(:last-child) {
      border-bottom: 0.5px solid var(--border-3);
    }

    &:hover {
      background-color: var(--surface-4);
    }

    &.reversed {
      flex-direction: row-reverse;
    }

    @include font-inter-400(14px, 20px, var(--text-secondary));
    @include flex(row, flex-start, center, 8px);
  }
</style>
