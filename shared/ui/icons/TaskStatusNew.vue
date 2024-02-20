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
      d="M3.3335 10.8333H16.3227C16.7081 10.8333 16.9008 10.8333 17.0109 10.7525C17.1068 10.682 17.1681 10.5739 17.1791 10.4553C17.1918 10.3194 17.0926 10.1541 16.8944 9.82367L15.206 7.00966C15.1311 6.88496 15.0937 6.8226 15.0791 6.75604C15.0662 6.69716 15.0662 6.63618 15.0791 6.5773C15.0937 6.51073 15.1311 6.44838 15.206 6.32367L16.8944 3.50966C17.0927 3.17919 17.1918 3.01395 17.1791 2.878C17.1681 2.75945 17.1068 2.65129 17.0109 2.58082C16.9008 2.5 16.7081 2.5 16.3227 2.5H3.3335L3.3335 17.5"
      stroke="#677383"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>

<style scoped lang="scss"></style>
