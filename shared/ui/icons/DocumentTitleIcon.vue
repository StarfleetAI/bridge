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
      d="M9 1.21563V4.52006C9 4.9681 9 5.19212 9.08719 5.36325C9.16389 5.51378 9.28628 5.63616 9.43681 5.71286C9.60794 5.80006 9.83196 5.80006 10.28 5.80006H13.5844M9 13H4.2M10.6 9.8H4.2M13.8 7.39058V13.16C13.8 14.5041 13.8 15.1762 13.5384 15.6896C13.3083 16.1412 12.9412 16.5083 12.4896 16.7384C11.9762 17 11.3041 17 9.96 17H4.84C3.49587 17 2.82381 17 2.31042 16.7384C1.85883 16.5083 1.49168 16.1412 1.26158 15.6896C1 15.1762 1 14.5041 1 13.16V4.84C1 3.49587 1 2.82381 1.26158 2.31042C1.49168 1.85883 1.85883 1.49168 2.31042 1.26158C2.82381 1 3.49587 1 4.84 1H7.40942C7.99644 1 8.28994 1 8.56615 1.06631C8.81104 1.1251 9.04515 1.22207 9.25988 1.35366C9.50208 1.50208 9.70962 1.70963 10.1247 2.12471L12.6753 4.67529C13.0904 5.09037 13.2979 5.29792 13.4463 5.54012C13.5779 5.75485 13.6749 5.98896 13.7337 6.23384C13.8 6.51005 13.8 6.80356 13.8 7.39058Z"
      :stroke="color"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
