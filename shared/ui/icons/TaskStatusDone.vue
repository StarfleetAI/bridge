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
      width: 21,
      height: 21,
      viewBoxValue: `0 0 21 21`,
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
      d="M8.01074 9.29167L10.5107 11.7917L18.8441 3.45833M13.8441 2.625H7.01074C5.61061 2.625 4.91055 2.625 4.37577 2.89748C3.90536 3.13717 3.52291 3.51962 3.28323 3.99002C3.01074 4.5248 3.01074 5.22487 3.01074 6.625V13.625C3.01074 15.0251 3.01074 15.7252 3.28323 16.26C3.52291 16.7304 3.90536 17.1128 4.37577 17.3525C4.91055 17.625 5.61061 17.625 7.01074 17.625H14.0107C15.4109 17.625 16.1109 17.625 16.6457 17.3525C17.1161 17.1128 17.4986 16.7304 17.7383 16.26C18.0107 15.7252 18.0107 15.0251 18.0107 13.625V10.125"
      stroke="#24864B"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
