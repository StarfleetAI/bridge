<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->
<script setup lang="ts">
  import { DefaultLayoutSidebar } from '~/widgets/layout/default'
  import { Modal } from '~/shared/ui/modal'
</script>
<template>
  <div class="default-layout">
    <Modal />
    <DefaultLayoutSidebar />
    <div class="default-layout__page">
      <slot name="default" />
    </div>
  </div>
</template>
<style scoped lang="scss">
  .default-layout {
    height: 100vh; // fallback for old browsers
    height: 100svh;

    @include flex(row, flex-start, stretch);
  }

  .default-layout__page {
    display: flex;
    flex: 1;
    overflow: hidden;
  }
</style>
