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
      width: 21,
      height: 21,
      viewBoxValue: `0 0 21 21`,
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
    <rect
      x="4.51074"
      y="4.125"
      width="12"
      height="12"
      rx="6"
      fill="#62AC55"
    />
    <rect
      x="3.76074"
      y="3.375"
      width="13.5"
      height="13.5"
      rx="6.75"
      stroke="#00A155"
      stroke-opacity="0.3"
      stroke-width="1.5"
    />
  </IconBase>
</template>
