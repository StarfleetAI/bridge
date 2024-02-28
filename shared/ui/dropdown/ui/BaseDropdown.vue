<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import 'floating-vue/dist/style.css'
  import { Dropdown } from 'floating-vue'
</script>

<template>
  <Dropdown placement="bottom-end">
    <slot />
    <template #popper>
      <slot name="content" />
    </template>
  </Dropdown>
</template>

<style lang="scss" scoped>
  :global(.v-popper__arrow-container) {
    display: none;
  }

  :global(.v-popper__inner) {
    border: none !important;
    background-color: var(--surface-3) !important;
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.55) !important;
  }
</style>
