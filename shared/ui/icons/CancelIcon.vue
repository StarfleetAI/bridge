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
      d="M4.10866 4.67311L15.892 16.4564M18.3337 10.5648C18.3337 15.1672 14.6027 18.8981 10.0003 18.8981C5.39795 18.8981 1.66699 15.1672 1.66699 10.5648C1.66699 5.96241 5.39795 2.23145 10.0003 2.23145C14.6027 2.23145 18.3337 5.96241 18.3337 10.5648Z"
      stroke="currentColor"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
