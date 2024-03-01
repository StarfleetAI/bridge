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
      viewBoxValue: `0 0 16 16`,
      width: 16,
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
      d="M4.33301 12.6668C2.67615 12.6668 1.33301 11.3237 1.33301 9.66683C1.33301 8.10445 2.52735 6.82102 4.05284 6.67974C4.36488 4.78158 6.01317 3.3335 7.99967 3.3335C9.98618 3.3335 11.6345 4.78158 11.9465 6.67974C13.472 6.82102 14.6663 8.10445 14.6663 9.66683C14.6663 11.3237 13.3232 12.6668 11.6663 12.6668C8.73979 12.6668 6.89521 12.6668 4.33301 12.6668Z"
      stroke="#677383"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
