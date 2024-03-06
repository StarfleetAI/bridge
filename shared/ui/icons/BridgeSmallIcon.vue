<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  withDefaults(
    defineProps<{
      width: string
      height: string
    }>(),
    {
      width: '24px',
      height: '24px',
    },
  )
</script>

<template>
  <img src="~/assets/img/bridge-small-icon.png" />
</template>

<style lang="scss" scoped>
  img {
    object-fit: cover;
    width: v-bind(width);
    height: v-bind(height);
  }
</style>
