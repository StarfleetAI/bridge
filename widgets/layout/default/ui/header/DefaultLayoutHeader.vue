<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import HeaderItem from './HeaderItem.vue'
</script>

<template>
  <div class="header__wrapper">
    <HeaderItem to="/" />
  </div>
</template>

<style scoped lang="scss">
  .sidebar__wrapper {
    position: relative;
    flex-shrink: 0;
    width: 288px;
    height: 100%;
    min-height: calc(100vh);
    min-height: calc(100svh);
    padding: 34px 24px 32px;
    background: var(--gray-200);

    @include flex(column, flex-start, flex-start);
  }
</style>
