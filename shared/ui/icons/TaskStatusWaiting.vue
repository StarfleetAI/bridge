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
      d="M13.7498 14.1665H17.9165L13.7498 18.3332H17.9165M18.292 10.8332C18.3192 10.5591 18.3332 10.2811 18.3332 9.99984C18.3332 5.39746 14.6022 1.6665 9.99984 1.6665C5.39746 1.6665 1.6665 5.39746 1.6665 9.99984C1.6665 14.6022 5.39746 18.3332 9.99984 18.3332C10.1396 18.3332 10.2785 18.3297 10.4165 18.3229C10.5563 18.316 10.6952 18.3057 10.8332 18.292M9.99984 4.99984V9.99984L13.1151 11.5575"
      stroke="#DA893E"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
