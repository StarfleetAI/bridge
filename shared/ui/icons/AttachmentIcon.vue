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
      d="M17.6011 5.25581V16.5C17.6011 19.5376 15.1386 22 12.1011 22C9.06351 22 6.60107 19.5376 6.60107 16.5V5.66667C6.60107 3.64162 8.2427 2 10.2677 2C12.2928 2 13.9344 3.64162 13.9344 5.66667V16.4457C13.9344 17.4583 13.1136 18.2791 12.1011 18.2791C11.0886 18.2791 10.2677 17.4583 10.2677 16.4457V6.65116"
      stroke="currentColor"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
