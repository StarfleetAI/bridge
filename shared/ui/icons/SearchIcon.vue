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
      height: 22,
      viewBoxValue: `0 0 23 22`,
      width: 23,
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
      d="M19.8008 19.25L14.3009 13.75M16.1341 9.16667C16.1341 12.7105 13.2613 15.5833 9.71745 15.5833C6.17362 15.5833 3.30078 12.7105 3.30078 9.16667C3.30078 5.62284 6.17362 2.75 9.71745 2.75C13.2613 2.75 16.1341 5.62284 16.1341 9.16667Z"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>

<style scoped lang="scss"></style>
