<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts"></script>

<template>
  <div class="only-content-layout">
    <slot />
  </div>
</template>

<style scoped lang="scss">
  .only-content-layout {
    height: 100%;
    min-height: calc(100vh - var(--header-height)); // fallback for old browsers
    min-height: calc(100svh - var(--header-height));
  }
</style>
