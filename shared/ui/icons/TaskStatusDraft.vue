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
      d="M9.16699 3.3332H5.66699C4.26686 3.3332 3.5668 3.3332 3.03202 3.60568C2.56161 3.84536 2.17916 4.22782 1.93948 4.69822C1.66699 5.233 1.66699 5.93307 1.66699 7.3332V14.3332C1.66699 15.7333 1.66699 16.4334 1.93948 16.9682C2.17916 17.4386 2.56161 17.821 3.03202 18.0607C3.5668 18.3332 4.26686 18.3332 5.66699 18.3332H12.667C14.0671 18.3332 14.7672 18.3332 15.302 18.0607C15.7724 17.821 16.1548 17.4386 16.3945 16.9682C16.667 16.4334 16.667 15.7333 16.667 14.3332V10.8332M6.66697 13.3332H8.06242C8.47007 13.3332 8.6739 13.3332 8.86571 13.2871C9.03577 13.2463 9.19835 13.179 9.34747 13.0876C9.51566 12.9845 9.65979 12.8404 9.94804 12.5521L17.917 4.5832C18.6073 3.89284 18.6073 2.77355 17.917 2.0832C17.2266 1.39284 16.1073 1.39284 15.417 2.0832L7.44802 10.0521C7.15977 10.3404 7.01564 10.4845 6.91257 10.6527C6.82119 10.8018 6.75385 10.9644 6.71302 11.1345C6.66697 11.3263 6.66697 11.5301 6.66697 11.9378V13.3332Z"
      stroke="currentColor"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
