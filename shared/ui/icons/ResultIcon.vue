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
      height: 21,
      viewBoxValue: `0 0 21 21`,
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
      d="M10.5106 2.4165L12.1872 6.77574C12.4222 7.38677 12.5397 7.69228 12.7225 7.94926C12.8844 8.17702 13.0834 8.37601 13.3112 8.53796C13.5681 8.72069 13.8737 8.8382 14.4847 9.07321L18.8439 10.7498L14.4847 12.4265C13.8737 12.6615 13.5681 12.779 13.3112 12.9617C13.0834 13.1237 12.8844 13.3227 12.7225 13.5504C12.5397 13.8074 12.4222 14.1129 12.1872 14.7239L10.5106 19.0832L8.83395 14.7239C8.59894 14.1129 8.48144 13.8074 8.29871 13.5504C8.13676 13.3227 7.93776 13.1237 7.71 12.9617C7.45302 12.779 7.14751 12.6615 6.53648 12.4265L2.17725 10.7498L6.53648 9.07321C7.14751 8.8382 7.45302 8.72069 7.71 8.53796C7.93776 8.37601 8.13676 8.17702 8.29871 7.94926C8.48144 7.69228 8.59894 7.38677 8.83395 6.77574L10.5106 2.4165Z"
      stroke="#C7CCD9"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
