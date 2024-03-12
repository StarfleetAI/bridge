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
      height: 21,
      viewBoxValue: `0 0 20 21`,
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
      d="M1.6665 8.83333C1.6665 8.83333 3.33732 6.55685 4.6947 5.19854C6.05208 3.84022 7.92783 3 9.99984 3C14.142 3 17.4998 6.35786 17.4998 10.5C17.4998 14.6421 14.142 18 9.99984 18C6.58059 18 3.69576 15.7119 2.79298 12.5833M1.6665 8.83333V3.83333M1.6665 8.83333H6.6665"
      stroke="currentColor"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
