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
      d="M9.67725 3.4582H6.17725C4.77711 3.4582 4.07705 3.4582 3.54227 3.73068C3.07186 3.97036 2.68941 4.35282 2.44973 4.82322C2.17725 5.358 2.17725 6.05807 2.17725 7.4582V14.4582C2.17725 15.8583 2.17725 16.5584 2.44973 17.0932C2.68941 17.5636 3.07186 17.946 3.54227 18.1857C4.07705 18.4582 4.77711 18.4582 6.17725 18.4582H13.1772C14.5774 18.4582 15.2774 18.4582 15.8122 18.1857C16.2826 17.946 16.6651 17.5636 16.9048 17.0932C17.1772 16.5584 17.1772 15.8583 17.1772 14.4582V10.9582M7.17722 13.4582H8.57268C8.98033 13.4582 9.18415 13.4582 9.37597 13.4121C9.54603 13.3713 9.7086 13.304 9.85772 13.2126C10.0259 13.1095 10.17 12.9654 10.4583 12.6771L18.4272 4.7082C19.1176 4.01784 19.1176 2.89855 18.4272 2.2082C17.7369 1.51784 16.6176 1.51784 15.9272 2.2082L7.95827 10.1771C7.67002 10.4654 7.52589 10.6095 7.42282 10.7777C7.33144 10.9268 7.2641 11.0894 7.22327 11.2595C7.17722 11.4513 7.17722 11.6551 7.17722 12.0628V13.4582Z"
      stroke="#C07EE8"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
