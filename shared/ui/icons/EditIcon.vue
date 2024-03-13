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
      height: 21,
      viewBoxValue: `0 0 20 21`,
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
      d="M14.9997 8.83326L11.6664 5.49993M2.08301 18.4166L4.90331 18.1032C5.24789 18.0649 5.42018 18.0458 5.58121 17.9937C5.72408 17.9474 5.86005 17.8821 5.98541 17.7994C6.12672 17.7062 6.2493 17.5836 6.49445 17.3385L17.4997 6.33326C18.4202 5.41279 18.4202 3.9204 17.4997 2.99993C16.5792 2.07945 15.0868 2.07945 14.1664 2.99992L3.16112 14.0052C2.91596 14.2503 2.79339 14.3729 2.70021 14.5142C2.61753 14.6396 2.55219 14.7755 2.50594 14.9184C2.4538 15.0794 2.43466 15.2517 2.39637 15.5963L2.08301 18.4166Z"
      stroke="currentColor"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
