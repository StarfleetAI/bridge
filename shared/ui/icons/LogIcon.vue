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
      d="M17.5 4.16699L8.33333 4.16699M17.5 15.8337L8.33333 15.8337M17.5 10.0003L8.33333 10.0003M5 4.16699C5 4.85735 4.44036 5.41699 3.75 5.41699C3.05964 5.41699 2.5 4.85735 2.5 4.16699C2.5 3.47664 3.05964 2.91699 3.75 2.91699C4.44036 2.91699 5 3.47664 5 4.16699ZM5 15.8337C5 16.524 4.44036 17.0837 3.75 17.0837C3.05964 17.0837 2.5 16.524 2.5 15.8337C2.5 15.1433 3.05964 14.5837 3.75 14.5837C4.44036 14.5837 5 15.1433 5 15.8337ZM5 10.0003C5 10.6907 4.44036 11.2503 3.75 11.2503C3.05964 11.2503 2.5 10.6907 2.5 10.0003C2.5 9.30997 3.05964 8.75033 3.75 8.75033C4.44036 8.75033 5 9.30997 5 10.0003Z"
      stroke="currentColor"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
