<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  defineOptions({ name: 'AuthLayout' })
</script>

<template>
  <div class="auth-layout">
    <slot />
  </div>
</template>

<style scoped lang="scss">
  .auth-layout {
    width: 100%;
    height: 100vh;

    @include flex(column, center, center);
  }
</style>
