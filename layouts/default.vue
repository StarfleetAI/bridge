<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { DefaultLayoutHeader } from '~/widgets/layout/default'
</script>

<template>
  <div class="default-layout">
    <DefaultLayoutHeader />
    <div class="default-layout__page">
      <slot name="default" />
    </div>
  </div>
</template>

<style scoped lang="scss">
  .default-layout {
    @include flex(column, flex-start, flex-start);
  }

  .default-layout__page {
    --header-height: 44px;
    flex: 1;
    height: 100%;
    min-height: calc(100vh - var(--header-height)); // fallback for old browsers
    min-height: calc(100svh - var(--header-height));
  }
</style>
