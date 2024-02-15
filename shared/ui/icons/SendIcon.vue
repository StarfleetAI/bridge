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
      viewBoxValue: `0 0 25 24`
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
      d="M12.5507 18.9998V11.9998M12.8422 19.0845L19.8207 21.4207C20.3676 21.6038 20.6411 21.6953 20.8097 21.6296C20.9562 21.5726 21.0672 21.45 21.1097 21.2987C21.1585 21.1244 21.0406 20.8613 20.8048 20.3349L13.3163 3.6222C13.0857 3.1075 12.9704 2.85015 12.8097 2.77041C12.6702 2.70113 12.5063 2.70087 12.3665 2.7697C12.2056 2.84892 12.0895 3.1059 11.8572 3.61986L4.30273 20.3359C4.06503 20.8619 3.94619 21.1248 3.99452 21.2993C4.03649 21.4508 4.14726 21.5737 4.29358 21.6312C4.46207 21.6974 4.73596 21.6064 5.28375 21.4246L12.3363 19.0838C12.4302 19.0527 12.4771 19.0371 12.5252 19.0309C12.5678 19.0255 12.6109 19.0255 12.6535 19.0311C12.7015 19.0374 12.7484 19.0531 12.8422 19.0845Z"
      stroke="currentColor"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </IconBase>
</template>

<style scoped lang="scss"></style>
