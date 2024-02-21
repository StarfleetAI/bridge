<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { CloudIcon, AbilitiesIcon } from '~/shared/ui/icons'
</script>
<template>
  <div class="abilities-list-item">
    <AbilitiesIcon />
    <div class="abilities-list-item__body">
      <div class="abilities-list-item__label">
        Frontend Developer
        <CloudIcon />
      </div>
      <div class="abilities-list-item__text">
        lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum
        lorem ipsum
      </div>
    </div>
  </div>
</template>
<style scoped lang="scss">
  .abilities-list-item {
    gap: 8px;
    margin-bottom: 12px;
    padding: 12px 16px;
    border-radius: 6px;
    background: var(--side-panel);

    &__label {
      gap: 8px;
      margin-bottom: 4px;

      @include flex(row, start, center);
      @include font-inter-700(16px, 22px, var(--text-primary));
    }

    &__text {
      @include font-inter-400(14px, 20px, var(--text-tertiary));
    }

    @include flex(row, start, space-between);
  }
</style>
