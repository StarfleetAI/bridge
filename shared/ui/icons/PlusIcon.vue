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
      height: 16,
      viewBoxValue: `0 0 17 16`,
      width: 17
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
    <path
      d="M8.55046 3.3335V12.6668M3.88379 8.00016H13.2171"
      stroke="currentColor"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>

<style scoped lang="scss"></style>
