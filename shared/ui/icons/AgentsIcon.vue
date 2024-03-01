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
      d="M6.80029 6.66667V7.91667M14.3003 6.66667V7.91667M9.71696 10.5001C10.3836 10.5001 10.967 9.91675 10.967 9.25008V6.66667M13.2171 12.6666C11.7171 14.1666 9.30046 14.1666 7.80046 12.6666M3.05029 6.5L3.05029 13.5C3.05029 14.9001 3.05029 15.6002 3.32278 16.135C3.56246 16.6054 3.94491 16.9878 4.41532 17.2275C4.9501 17.5 5.65016 17.5 7.05029 17.5H14.0503C15.4504 17.5 16.1505 17.5 16.6853 17.2275C17.1557 16.9878 17.5381 16.6054 17.7778 16.135C18.0503 15.6002 18.0503 14.9001 18.0503 13.5V6.5C18.0503 5.09987 18.0503 4.3998 17.7778 3.86502C17.5381 3.39462 17.1557 3.01217 16.6853 2.77248C16.1505 2.5 15.4504 2.5 14.0503 2.5L7.05029 2.5C5.65016 2.5 4.9501 2.5 4.41532 2.77248C3.94491 3.01217 3.56246 3.39462 3.32278 3.86502C3.05029 4.3998 3.05029 5.09987 3.05029 6.5Z"
      stroke="currentColor"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
