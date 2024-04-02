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
      d="M3.33301 3.7671C3.33301 3.11965 3.33301 2.79593 3.468 2.61749C3.5856 2.46203 3.76536 2.36583 3.95994 2.35421C4.1833 2.34087 4.45265 2.52044 4.99135 2.87958L12.0017 7.55316C12.4469 7.84991 12.6694 7.99828 12.747 8.1853C12.8148 8.3488 12.8148 8.53256 12.747 8.69606C12.6694 8.88307 12.4469 9.03145 12.0017 9.3282L4.99135 14.0018C4.45265 14.3609 4.1833 14.5405 3.95994 14.5271C3.76536 14.5155 3.5856 14.4193 3.468 14.2639C3.33301 14.0854 3.33301 13.7617 3.33301 13.1143V3.7671Z"
      stroke="currentColor"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>
