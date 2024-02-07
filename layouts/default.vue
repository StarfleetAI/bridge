<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { DefaultLayoutHeader } from '~/widgets/layout/default'
</script>

<template>
  <div class="default-layout__container">
    <div class="default-layout__page-wrapper">
      <DefaultLayoutHeader />
      <div class="default-layout__page-container">
        <slot name="default" />
      </div>
    </div>
  </div>
</template>

<style lang="scss">
  body {
    margin: 0;
    padding: 0;
  }
  .default-layout__container {
    position: relative;
    box-sizing: border-box;
    background-color: var(--surface-secondary);

    @include flex(column, flex-start, flex-start);
  }
  .default-layout__page-wrapper {
    width: 100%;

    @include flex(row, flex-start, stretch);
  }
  .default-layout__page-container {
    z-index: 2;
    flex: 1;
    max-width: calc(100vw - 288px);
    height: 100%;
    min-height: calc(100vh - 64px); // fallback for old browsers
    min-height: calc(100svh - 64px);
  }
</style>
