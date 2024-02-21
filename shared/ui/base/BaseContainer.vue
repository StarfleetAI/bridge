<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup></script>

<template>
  <div class="base-container">
    <div class="base-container__main">
      <slot name="main" />
    </div>
    <div
      v-if="$slots.additional"
      class="base-container__additional"
    >
      <slot name="additional" />
    </div>
  </div>
</template>
<style lang="scss" scoped>
  .base-container {
    width: 100%;

    @include flex(row, flex-start, stretch);
  }

  .base-container__main {
    display: flex;
    flex: 1;
    height: 100%;
  }

  .base-container__additional {
    width: 40%;
  }
</style>
