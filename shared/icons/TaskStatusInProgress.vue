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
      height: 12,
      viewBoxValue: `0 0 12 12`,
      width: 12
    }
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
      x="2"
      y="2"
      width="12"
      height="12"
      rx="6"
      fill="#62AC55"
    />
    <rect
      x="1.25"
      y="1.25"
      width="13.5"
      height="13.5"
      rx="6.75"
      stroke="#00A155"
      stroke-opacity="0.3"
      stroke-width="1.5"
    />
  </IconBase>
</template>

<style scoped lang="scss"></style>
