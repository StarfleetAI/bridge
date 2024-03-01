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
      d="M12.2171 1.89111V5.33323C12.2171 5.79994 12.2171 6.0333 12.308 6.21156C12.3878 6.36836 12.5153 6.49584 12.6721 6.57574C12.8504 6.66656 13.0837 6.66656 13.5505 6.66656H16.9926M12.2171 14.1665H7.21712M13.8838 10.8332H7.21712M17.2171 8.32336V14.3332C17.2171 15.7333 17.2171 16.4334 16.9446 16.9681C16.705 17.4386 16.3225 17.821 15.8521 18.0607C15.3173 18.3332 14.6173 18.3332 13.2171 18.3332H7.88379C6.48366 18.3332 5.78359 18.3332 5.24881 18.0607C4.77841 17.821 4.39596 17.4386 4.15627 16.9681C3.88379 16.4334 3.88379 15.7333 3.88379 14.3332V5.6665C3.88379 4.26637 3.88379 3.56631 4.15627 3.03153C4.39596 2.56112 4.77841 2.17867 5.24881 1.93899C5.78359 1.6665 6.48366 1.6665 7.88379 1.6665H10.5603C11.1717 1.6665 11.4775 1.6665 11.7652 1.73558C12.0203 1.79682 12.2642 1.89783 12.4878 2.0349C12.7401 2.18951 12.9563 2.4057 13.3887 2.83808L16.0455 5.49493C16.4779 5.92731 16.6941 6.1435 16.8487 6.39579C16.9858 6.61947 17.0868 6.86333 17.148 7.11842C17.2171 7.40614 17.2171 7.71188 17.2171 8.32336Z"
      stroke="currentColor"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
