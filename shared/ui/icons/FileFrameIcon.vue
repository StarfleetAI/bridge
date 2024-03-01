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
      height: 44,
      viewBoxValue: `0 0 32 44`,
      width: 32,
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
      d="M0 4C0 1.79086 1.79086 0 4 0H16H24L32 9V40C32 42.2091 30.2091 44 28 44H4C1.79086 44 0 42.2091 0 40V4Z"
      fill="#464B59"
    />
  </IconBase>
</template>
