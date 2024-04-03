<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  withDefaults(
    defineProps<{
      type?: 'solid' | 'outline'
      color?: 'blue' | 'green' | 'red' | 'gray'
      shade?: 'default' | 'soft'
      disabled?: boolean
      size?: 'large' | 'medium' | 'small' | 'xsmall'
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
  <button :class="['base-button', color, shade, size, type, { disabled }]">
    <slot name="icon" />
    <span><slot /></span>
  </button>
</template>

<style lang="scss">
  .base-button {
    position: relative;
    overflow: hidden;
    border: none;
    border-radius: 6px;
    box-shadow: none;
    transition:
      background-color 0.2s ease-in-out,
      color 0.2s ease-in-out;

    & * {
      position: relative;
      z-index: 2;
    }

    &:before {
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      z-index: 1;
      display: block;
      width: 100%;
      height: 100%;
      background: #000;
      opacity: 0;
    }

    &:hover {
      &:before {
        opacity: 0.2;
      }
    }

    &.blue {
      background-color: var(--button-primary);
      color: var(--text-on-button);

      &.soft {
        background-color: rgba(68, 75, 209, 0.1);
        color: var(--button-secondary) !important;

        &:before {
          display: none !important;
        }

        &:hover {
          background-color: rgba(68, 75, 209, 0.2);
        }
      }
    }

    &.green {
      background-color: var(--status-done);
      color: var(--text-on-button);

      &.soft {
        background-color: rgba(36, 134, 75, 0.1);
        color: var(--status-done) !important;

        &:before {
          display: none !important;
        }

        &:hover {
          background-color: rgba(36, 134, 75, 0.2) !important;
        }
      }
    }

    &.red {
      background-color: var(--status-failed);
      color: var(--text-on-button);

      &.soft {
        background-color: rgba(183, 78, 71, 0.1);
        color: var(--status-failed) !important;

        &:before {
          display: none !important;
        }

        &:hover {
          background-color: rgba(183, 78, 71, 0.2);
        }
      }
    }

    &.gray {
      background-color: var(--surface-4);

      span {
        color: var(--text-secondary);
      }
    }

    &.large {
      padding: 8px 12px;

      svg {
        width: 20px;
        height: 20px;
      }

      @include font-inter-500(14px, 20px, var(--text-on-button));
    }

    &.medium {
      padding: 6px 10px;

      @include font-inter-500(14px, 20px, var(--text-on-button));
    }

    &.small {
      padding: 4px 8px;

      @include font-inter-500(14px, 20px, var(--text-on-button));
    }

    &.xsmall {
      padding: 4px 8px;

      @include font-inter-400(12px, 17px, var(--text-on-button));
    }

    &.outline {
      background: none !important;
      color: var(--text-tertiary);
      outline: 1px solid var(--border-2);

      &:hover {
        color: var(--text-secondary);
      }
    }

    &.disabled {
      opacity: 0.3;
      pointer-events: none;
    }

    svg {
      width: 16px;
      height: 16px;
    }

    @include flex(row, center, center, $gap: 6px);
  }
</style>
