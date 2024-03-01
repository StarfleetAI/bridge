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
      width: 25,
      height: 24,
      viewBoxValue: `0 0 25 24`,
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
      d="M21.703 10.8995L12.6874 19.9151C10.6372 21.9653 7.31304 21.9653 5.26279 19.9151C3.21253 17.8648 3.21253 14.5407 5.26279 12.4904L14.2784 3.47483C15.6452 2.108 17.8613 2.108 19.2281 3.47483C20.595 4.84167 20.595 7.05775 19.2281 8.42458L10.5661 17.0866C9.88267 17.7701 8.77463 17.7701 8.09121 17.0866C7.4078 16.4032 7.4078 15.2952 8.09121 14.6118L15.6926 7.01037"
      stroke="currentColor"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
