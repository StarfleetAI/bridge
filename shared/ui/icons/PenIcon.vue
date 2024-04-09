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
      width: 20,
      height: 20,
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
      d="M2.39637 15.0963C2.43466 14.7517 2.4538 14.5794 2.50594 14.4184C2.55219 14.2755 2.61753 14.1396 2.70021 14.0142C2.79339 13.8729 2.91596 13.7503 3.16112 13.5052L14.1664 2.49992C15.0868 1.57945 16.5792 1.57945 17.4997 2.49993C18.4202 3.4204 18.4202 4.91279 17.4997 5.83326L6.49445 16.8385C6.2493 17.0836 6.12672 17.2062 5.98541 17.2994C5.86005 17.3821 5.72408 17.4474 5.58121 17.4937C5.42018 17.5458 5.24789 17.5649 4.90331 17.6032L2.08301 17.9166L2.39637 15.0963Z"
      stroke="currentColor"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
