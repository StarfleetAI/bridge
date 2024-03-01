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
      height: 26,
      viewBoxValue: `0 0 26 26`,
      width: 26,
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
      x="0.5"
      y="0.5"
      width="25"
      height="25"
      rx="12.5"
      fill="#646BF0"
    />
    <rect
      x="0.5"
      y="0.5"
      width="25"
      height="25"
      rx="12.5"
      stroke="#272B36"
    />
    <path
      d="M13.0002 15C10.8868 15 9.00742 16.0204 7.81089 17.604C7.55336 17.9448 7.4246 18.1152 7.42881 18.3455C7.43207 18.5235 7.5438 18.7479 7.6838 18.8578C7.86502 19 8.11614 19 8.61838 19H17.3821C17.8843 19 18.1354 19 18.3166 18.8578C18.4567 18.7479 18.5684 18.5235 18.5716 18.3455C18.5759 18.1152 18.4471 17.9448 18.1896 17.604C16.993 16.0204 15.1136 15 13.0002 15Z"
      fill="#3940CB"
    />
    <path
      d="M13.0002 13C14.6571 13 16.0002 11.6569 16.0002 10C16.0002 8.34315 14.6571 7 13.0002 7C11.3434 7 10.0002 8.34315 10.0002 10C10.0002 11.6569 11.3434 13 13.0002 13Z"
      fill="#3940CB"
    />
  </IconBase>
</template>
