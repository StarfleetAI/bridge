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
      viewBoxValue: `0 0 21 20`,
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
      d="M10.4167 1.6665H12.6667C14.0668 1.6665 14.7669 1.6665 15.3016 1.93899C15.772 2.17867 16.1545 2.56112 16.3942 3.03153C16.6667 3.56631 16.6667 4.26637 16.6667 5.6665V14.3332C16.6667 15.7333 16.6667 16.4334 16.3942 16.9681C16.1545 17.4386 15.772 17.821 15.3016 18.0607C14.7669 18.3332 14.0668 18.3332 12.6667 18.3332H7.33333C5.9332 18.3332 5.23314 18.3332 4.69836 18.0607C4.22795 17.821 3.8455 17.4386 3.60582 16.9681C3.33333 16.4334 3.33333 15.7333 3.33333 14.3332V13.7498M13.3333 10.8332H9.58333M13.3333 7.49984H10.4167M13.3333 14.1665H6.66667M5 8.33317V3.74984C5 3.05948 5.55964 2.49984 6.25 2.49984C6.94036 2.49984 7.5 3.05948 7.5 3.74984V8.33317C7.5 9.71388 6.38071 10.8332 5 10.8332C3.61929 10.8332 2.5 9.71388 2.5 8.33317V4.99984"
      stroke="#677383"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
