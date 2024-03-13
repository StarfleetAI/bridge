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

    &.reversed {
      flex-direction: row-reverse;
    }

    @include flex(row, flex-start, center, 8px);
  }
</style>
