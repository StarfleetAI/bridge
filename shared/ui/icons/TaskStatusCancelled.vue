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
    <path
      d="M13.0106 7.62484L8.01058 12.6248M8.01058 7.62484L13.0106 12.6248M18.8439 10.1248C18.8439 14.7272 15.113 18.4582 10.5106 18.4582C5.90821 18.4582 2.17725 14.7272 2.17725 10.1248C2.17725 5.52246 5.90821 1.7915 10.5106 1.7915C15.113 1.7915 18.8439 5.52246 18.8439 10.1248Z"
      stroke="#677383"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
