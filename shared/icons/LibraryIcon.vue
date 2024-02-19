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
      viewBoxValue: `0 0 20 20`,
      width: 20
    }
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
      d="M10.0003 16.6668H4.33366C3.40024 16.6668 2.93353 16.6668 2.57701 16.4852C2.2634 16.3254 2.00844 16.0704 1.84865 15.7568C1.66699 15.4003 1.66699 14.9336 1.66699 14.0002V6.00016C1.66699 5.06674 1.66699 4.60003 1.84865 4.24351C2.00844 3.92991 2.2634 3.67494 2.57701 3.51515C2.93353 3.3335 3.40024 3.3335 4.33366 3.3335H4.66699C6.53383 3.3335 7.46725 3.3335 8.18029 3.69681C8.8075 4.01639 9.31744 4.52632 9.63701 5.15353C10.0003 5.86657 10.0003 6.79999 10.0003 8.66683M10.0003 16.6668V8.66683M10.0003 16.6668H15.667C16.6004 16.6668 17.0671 16.6668 17.4236 16.4852C17.7372 16.3254 17.9922 16.0704 18.152 15.7568C18.3337 15.4003 18.3337 14.9336 18.3337 14.0002V6.00016C18.3337 5.06674 18.3337 4.60003 18.152 4.24351C17.9922 3.92991 17.7372 3.67494 17.4236 3.51515C17.0671 3.3335 16.6004 3.3335 15.667 3.3335H15.3337C13.4668 3.3335 12.5334 3.3335 11.8204 3.69681C11.1932 4.01639 10.6832 4.52632 10.3636 5.15353C10.0003 5.86657 10.0003 6.79999 10.0003 8.66683"
      stroke="#B0B5C0"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>

<style scoped lang="scss"></style>
