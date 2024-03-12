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
      d="M3.33268 13.5352C2.32769 12.8625 1.66602 11.7168 1.66602 10.4167C1.66602 8.46369 3.15894 6.85941 5.0658 6.68281C5.45586 4.31011 7.51622 2.5 9.99935 2.5C12.4825 2.5 14.5428 4.31011 14.9329 6.68281C16.8398 6.85941 18.3327 8.46369 18.3327 10.4167C18.3327 11.7168 17.671 12.8625 16.666 13.5352M6.66602 14.1667L9.99935 17.5M9.99935 17.5L13.3327 14.1667M9.99935 17.5V10"
      stroke="currentColor"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
