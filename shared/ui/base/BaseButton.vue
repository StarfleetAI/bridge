<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  defineProps<{
    type: 'primary' | 'secondary' | 'approve' | 'decline' | 'update'
    disabled: boolean
    outlined?: boolean
    size: 'small' | 'medium'
  }>()
</script>

<template>
  <div :class="['base-button', type, { disabled, outlined }]">
    <slot name="icon" />
  </div>
</template>

<style lang="scss" scoped>
  .base-button {
    height: 28px;
    padding: 4px 11px 4px 8px;
    border-radius: 4px;
    background-color: var(--button-primary);

    &.disabled {
      opacity: 0.3;
      pointer-events: none;
    }

    @include flex(row, center, center, $gap: 8px);
    @include font-inter-500(14px, 20px, var(--text-on-button));
  }
</style>
