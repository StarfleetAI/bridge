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
      width: 20,
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
      d="M7.50033 3.33317V1.6665M12.5003 16.6665V18.3332M3.33366 7.49984H1.66699M16.667 12.4998H18.3337M4.0955 4.09502L2.91699 2.9165M15.9051 15.9047L17.0837 17.0832M10.0003 14.7139L8.23256 16.4816C6.93081 17.7834 4.82026 17.7834 3.51851 16.4816C2.21677 15.1799 2.21677 13.0694 3.51851 11.7676L5.28628 9.99984M14.7144 9.99984L16.4821 8.23207C17.7839 6.93032 17.7839 4.81977 16.4821 3.51803C15.1804 2.21628 13.0698 2.21628 11.7681 3.51803L10.0003 5.28579"
      stroke="currentColor"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
