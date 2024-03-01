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
      width: 23,
      height: 22,
      viewBoxValue: `0 0 23 22`,
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
      d="M20.7171 8.25H2.38379M13.3838 16.0417L15.6755 13.75L13.3838 11.4583M9.71712 11.4583L7.42546 13.75L9.71712 16.0417M2.38379 7.15L2.38379 14.85C2.38379 16.3901 2.38379 17.1602 2.68352 17.7485C2.94717 18.2659 3.36787 18.6866 3.88531 18.9503C4.47357 19.25 5.24364 19.25 6.78379 19.25H16.3171C17.8573 19.25 18.6273 19.25 19.2156 18.9503C19.733 18.6866 20.1537 18.2659 20.4174 17.7485C20.7171 17.1602 20.7171 16.3901 20.7171 14.85V7.15C20.7171 5.60986 20.7171 4.83978 20.4174 4.25153C20.1537 3.73408 19.733 3.31339 19.2156 3.04973C18.6273 2.75 17.8573 2.75 16.3171 2.75L6.78379 2.75C5.24365 2.75 4.47357 2.75 3.88532 3.04973C3.36787 3.31338 2.94717 3.73408 2.68352 4.25153C2.38379 4.83978 2.38379 5.60986 2.38379 7.15Z"
      stroke="currentColor"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
