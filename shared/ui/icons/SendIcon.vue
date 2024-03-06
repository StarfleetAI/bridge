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
      width: 25,
      height: 24,
      viewBoxValue: `0 0 25 24`,
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
      d="M10.6011 13.5002L21.1011 3.00017M10.7287 13.8282L13.3568 20.5862C13.5883 21.1816 13.7041 21.4793 13.8709 21.5662C14.0155 21.6415 14.1877 21.6416 14.3324 21.5664C14.4993 21.4797 14.6154 21.1822 14.8476 20.5871L21.438 3.69937C21.6476 3.16219 21.7524 2.8936 21.6951 2.72197C21.6453 2.57292 21.5283 2.45596 21.3793 2.40616C21.2077 2.34883 20.9391 2.45364 20.4019 2.66327L3.51414 9.25361C2.91907 9.48584 2.62153 9.60195 2.53482 9.76886C2.45965 9.91354 2.45975 10.0858 2.53509 10.2304C2.622 10.3972 2.91967 10.513 3.51502 10.7445L10.273 13.3726C10.3939 13.4196 10.4543 13.4431 10.5052 13.4794C10.5503 13.5115 10.5897 13.551 10.6219 13.5961C10.6582 13.647 10.6817 13.7074 10.7287 13.8282Z"
      stroke="currentColor"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
