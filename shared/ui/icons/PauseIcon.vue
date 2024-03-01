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
      d="M7.91667 13.0645V8.06445M12.0833 13.0645V8.06445M6.5 18.0645H13.5C14.9001 18.0645 15.6002 18.0645 16.135 17.792C16.6054 17.5523 16.9878 17.1698 17.2275 16.6994C17.5 16.1646 17.5 15.4646 17.5 14.0645V7.06445C17.5 5.66432 17.5 4.96426 17.2275 4.42948C16.9878 3.95907 16.6054 3.57662 16.135 3.33694C15.6002 3.06445 14.9001 3.06445 13.5 3.06445H6.5C5.09987 3.06445 4.3998 3.06445 3.86502 3.33694C3.39462 3.57662 3.01217 3.95907 2.77248 4.42948C2.5 4.96426 2.5 5.66432 2.5 7.06445V14.0645C2.5 15.4646 2.5 16.1646 2.77248 16.6994C3.01217 17.1698 3.39462 17.5523 3.86502 17.792C4.3998 18.0645 5.09987 18.0645 6.5 18.0645Z"
      stroke="currentColor"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
