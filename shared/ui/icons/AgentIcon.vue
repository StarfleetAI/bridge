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
      color: '#677383',
      height: 16,
      viewBoxValue: `0 0 17 16`,
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
      d="M4.75 4.66667V5.91667M12.25 4.66667V5.91667M7.66667 8.50008C8.33333 8.50008 8.91667 7.91675 8.91667 7.25008V4.66667M11.1668 10.6666C9.66683 12.1666 7.25016 12.1666 5.75016 10.6666M1 4.5L1 11.5C1 12.9001 1 13.6002 1.27248 14.135C1.51217 14.6054 1.89462 14.9878 2.36502 15.2275C2.8998 15.5 3.59987 15.5 5 15.5H12C13.4001 15.5 14.1002 15.5 14.635 15.2275C15.1054 14.9878 15.4878 14.6054 15.7275 14.135C16 13.6002 16 12.9001 16 11.5V4.5C16 3.09987 16 2.3998 15.7275 1.86502C15.4878 1.39462 15.1054 1.01217 14.635 0.772485C14.1002 0.500001 13.4001 0.500001 12 0.500001L5 0.5C3.59987 0.5 2.8998 0.5 2.36502 0.772483C1.89462 1.01217 1.51217 1.39462 1.27248 1.86502C1 2.3998 1 3.09987 1 4.5Z"
      :stroke="color"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
