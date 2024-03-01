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
      color: '#B0B5C0',
      height: 16,
      viewBoxValue: `0 0 16 17`,
      width: 17,
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
      d="M9.33366 2.01314V4.76683C9.33366 5.1402 9.33366 5.32688 9.40632 5.46949C9.47024 5.59493 9.57222 5.69692 9.69766 5.76083C9.84027 5.8335 10.027 5.8335 10.4003 5.8335H13.154M13.3337 7.15898V11.9668C13.3337 13.0869 13.3337 13.647 13.1157 14.0748C12.9239 14.4511 12.618 14.7571 12.2416 14.9488C11.8138 15.1668 11.2538 15.1668 10.1337 15.1668H5.86699C4.74689 15.1668 4.18683 15.1668 3.75901 14.9488C3.38269 14.7571 3.07673 14.4511 2.88498 14.0748C2.66699 13.647 2.66699 13.0869 2.66699 11.9668V5.0335C2.66699 3.91339 2.66699 3.35334 2.88498 2.92552C3.07673 2.54919 3.38269 2.24323 3.75901 2.05148C4.18683 1.8335 4.74689 1.8335 5.86699 1.8335H8.00818C8.49736 1.8335 8.74195 1.8335 8.97212 1.88876C9.17619 1.93775 9.37128 2.01856 9.55023 2.12822C9.75206 2.2519 9.92501 2.42485 10.2709 2.77075L12.3964 4.89624C12.7423 5.24214 12.9153 5.41509 13.0389 5.61693C13.1486 5.79587 13.2294 5.99096 13.2784 6.19503C13.3337 6.42521 13.3337 6.6698 13.3337 7.15898Z"
      :stroke="color"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
