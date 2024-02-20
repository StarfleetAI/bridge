<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  const props = withDefaults(
    defineProps<{
      color?: string
      height?: number | string
      pointer?: boolean
      viewBoxValue?: string
      width?: number | string
    }>(),
    {
      color: 'currentColor',
      height: 24,
      viewBoxValue: `0 0 24 24`,
      width: 24,
    },
  )
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    :width="width"
    :height="height"
    :viewBox="props.viewBoxValue"
    :class="{ 'cursor-pointer': props.pointer }"
  >
    <g fill="none">
      <slot />
    </g>
  </svg>
</template>

<style lang="scss" scoped>
  svg {
    display: inline-block;
    vertical-align: baseline;
    color: v-bind('color');

    &.cursor-pointer {
      cursor: pointer;
    }
  }
</style>
