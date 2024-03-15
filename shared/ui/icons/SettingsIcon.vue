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
      height: 20,
      viewBoxValue: `0 0 25 25`,
      width: 21,
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
      d="M3.44995 8.5L15.45 8.5M15.45 8.5C15.45 10.1569 16.7931 11.5 18.45 11.5C20.1068 11.5 21.45 10.1569 21.45 8.5C21.45 6.84315 20.1068 5.5 18.45 5.5C16.7931 5.5 15.45 6.84315 15.45 8.5ZM9.44995 16.5L21.45 16.5M9.44995 16.5C9.44995 18.1569 8.10681 19.5 6.44995 19.5C4.7931 19.5 3.44995 18.1569 3.44995 16.5C3.44995 14.8431 4.7931 13.5 6.44995 13.5C8.10681 13.5 9.44995 14.8431 9.44995 16.5Z"
      stroke="currentColor"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
