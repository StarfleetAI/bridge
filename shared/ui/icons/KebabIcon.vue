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
      d="M10.0003 10.8335C10.4606 10.8335 10.8337 10.4604 10.8337 10.0002C10.8337 9.53993 10.4606 9.16683 10.0003 9.16683C9.54009 9.16683 9.16699 9.53993 9.16699 10.0002C9.16699 10.4604 9.54009 10.8335 10.0003 10.8335Z"
      stroke="currentColor"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
    <path
      d="M10.0003 5.00016C10.4606 5.00016 10.8337 4.62707 10.8337 4.16683C10.8337 3.70659 10.4606 3.3335 10.0003 3.3335C9.54009 3.3335 9.16699 3.70659 9.16699 4.16683C9.16699 4.62707 9.54009 5.00016 10.0003 5.00016Z"
      stroke="currentColor"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
    <path
      d="M10.0003 16.6668C10.4606 16.6668 10.8337 16.2937 10.8337 15.8335C10.8337 15.3733 10.4606 15.0002 10.0003 15.0002C9.54009 15.0002 9.16699 15.3733 9.16699 15.8335C9.16699 16.2937 9.54009 16.6668 10.0003 16.6668Z"
      stroke="currentColor"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
