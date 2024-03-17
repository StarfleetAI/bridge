<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  withDefaults(
    defineProps<{
      color?: 'blue' | 'orange' | 'gray' | 'dark'
      disabled?: boolean
      size?: 'medium' | 'small'
    }>(),
    {
      type: 'solid',
      color: 'blue',
      shade: 'default',
      disabled: false,
      size: 'medium',
    },
  )
</script>

<template>
  <div :class="['link', color, size, { disabled }]">
    <slot name="icon" />
    <span><slot /></span>
  </div>
</template>

<style lang="scss">
  .link {
    position: relative;
    display: inline-block;
    overflow: hidden;
    transition: color 0.2s ease-in-out;

    &:hover {
      opacity: 0.8;
    }

    &.blue {
      color: var(--button-secondary);
    }

    &.orange {
      color: var(--status-waiting);
    }

    &.gray {
      color: var(--text-secondary);
    }

    &.dark {
      color: var(--text-tertiary);
    }

    &.small {
      @include font-inter-500(14px, 20px);
    }

    svg {
      width: 16px;
      height: 16px;
    }

    @include font-inter-500(12px, 17px, var(--button-secondary));
    @include flex(row, center, center, $gap: 6px);
  }
</style>
