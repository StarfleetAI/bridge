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
    width: 21
  }
)
</script>

<template>
  <IconBase :width="width" :height="height" :color="color" :view-box-value="viewBoxValue" :pointer="pointer">
    <path
      d="M7.91667 12.5V7.5M12.0833 12.5V7.5M6.5 17.5H13.5C14.9001 17.5 15.6002 17.5 16.135 17.2275C16.6054 16.9878 16.9878 16.6054 17.2275 16.135C17.5 15.6002 17.5 14.9001 17.5 13.5V6.5C17.5 5.09987 17.5 4.3998 17.2275 3.86502C16.9878 3.39462 16.6054 3.01217 16.135 2.77248C15.6002 2.5 14.9001 2.5 13.5 2.5H6.5C5.09987 2.5 4.3998 2.5 3.86502 2.77248C3.39462 3.01217 3.01217 3.39462 2.77248 3.86502C2.5 4.3998 2.5 5.09987 2.5 6.5V13.5C2.5 14.9001 2.5 15.6002 2.77248 16.135C3.01217 16.6054 3.39462 16.9878 3.86502 17.2275C4.3998 17.5 5.09987 17.5 6.5 17.5Z"
      stroke="#9E83EA"
stroke-linecap="round"
stroke-linejoin="round" />
  </IconBase>
</template>

<style scoped lang="scss">
</style>
