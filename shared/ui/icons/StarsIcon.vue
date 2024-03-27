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
      width: 16,
      height: 17,
      viewBoxValue: `0 0 16 17`,
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
      d="M7.99967 1.44141L9.34098 4.9288C9.52899 5.41762 9.62299 5.66203 9.76917 5.86761C9.89873 6.04982 10.0579 6.20901 10.2401 6.33857C10.4457 6.48476 10.6901 6.57876 11.179 6.76677L14.6663 8.10807L11.179 9.44938C10.6901 9.63738 10.4457 9.73139 10.2401 9.87757C10.0579 10.0071 9.89873 10.1663 9.76917 10.3485C9.62299 10.5541 9.52899 10.7985 9.34098 11.2873L7.99968 14.7747L6.65837 11.2873C6.47036 10.7985 6.37636 10.5541 6.23018 10.3485C6.10062 10.1663 5.94142 10.0071 5.75921 9.87757C5.55363 9.73139 5.30922 9.63739 4.8204 9.44938L1.33301 8.10807L4.8204 6.76677C5.30922 6.57876 5.55363 6.48476 5.75921 6.33857C5.94142 6.20901 6.10062 6.04982 6.23018 5.86761C6.37636 5.66203 6.47036 5.41762 6.65837 4.9288L7.99967 1.44141Z"
      stroke="#EAECF0"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
