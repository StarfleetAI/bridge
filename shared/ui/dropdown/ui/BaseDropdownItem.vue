<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup></script>

<template>
  <div class="dropdown-item">
    <slot name="icon" />
    <slot name="label" />
  </div>
</template>

<style lang="scss" scoped>
  .dropdown-item {
    padding: 12px 16px;

    @include flex(row, flex-start, center, 8px);
  }
</style>
