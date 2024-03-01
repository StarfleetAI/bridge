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
      d="M7.5 3.06445H12.5M2.5 5.56445H17.5M15.8333 5.56445L15.2489 14.3305C15.1612 15.6457 15.1174 16.3033 14.8333 16.802C14.5833 17.2409 14.206 17.5939 13.7514 17.8142C13.235 18.0645 12.5759 18.0645 11.2578 18.0645H8.74221C7.42409 18.0645 6.76503 18.0645 6.24861 17.8142C5.79396 17.5939 5.41674 17.2409 5.16665 16.802C4.88259 16.3033 4.83875 15.6457 4.75107 14.3305L4.16667 5.56445"
      stroke="currentColor"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
