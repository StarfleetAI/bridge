<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import IconBase from './IconBase.vue'

  withDefaults(
    defineProps<{
      color?: string
      height?: number | string
      pointer?: boolean
      viewBoxValue?: string
      width?: number | string
    }>(),
    {
      color: 'currentColor',
      width: 20,
      height: 20,
      viewBoxValue: `0 0 20 20`,
    },
  )
</script>

<template>
  <IconBase
    :width="width"
    :height="height"
    :color="color"
    :view-box-value="viewBoxValue"
    :pointer="pointer"
  >
    <path
      d="M5 7.5L10 12.5L15 7.5"
      stroke="currentColor"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
