<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  withDefaults(
    defineProps<{
      type?: 'primary' | 'secondary' | 'approve' | 'decline' | 'update'
      disabled?: boolean
      outlined?: boolean
      size?: 'small' | 'medium'
    }>(),
    {
      type: 'primary',
      disabled: false,
      outlined: false,
      size: 'medium',
    },
  )
</script>

<template>
  <div :class="['base-button', type, size, { disabled, outlined }]">
    <slot name="icon" />
    <slot />
  </div>
</template>

<style lang="scss" scoped>
  .base-button {
    border-radius: 4px;

    &.primary {
      background-color: var(--button-primary);
      color: var(--text-on-button);
    }

    &.secondary {
      background-color: var(--surface-4);
      color: var(--text-secondary) !important;
    }

    &.outlined {
      border: 1px solid var(--border-2);
      background-color: transparent !important;
      color: var(--text-tertiary) !important;

      &:hover {
        color: var(--text-secondary) !important;
      }
    }

    &.disabled {
      opacity: 0.3;
      pointer-events: none;
    }

    &.small {
      padding: 4px 10px;

      @include font-inter-500(12px, 17px, var(--text-on-button));
    }

    &.medium {
      padding: 6px 10px;

      @include font-inter-500(14px, 20px, var(--text-on-button));
    }

    @include flex(row, center, center, $gap: 8px);
  }
</style>
