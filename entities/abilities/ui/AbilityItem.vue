<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import type { Ability } from '../model'

  defineProps<{
    ability: Ability
  }>()
</script>
<template>
  <div class="abilities-list-item">
    <div class="abilities-list-item__body">
      <div class="abilities-list-item__label">
        {{ ability.name }}
      </div>
      <div class="abilities-list-item__text">
        {{ ability.description }}
      </div>
      <div class="abilities-list-item__extra">by example.com</div>
    </div>
  </div>
</template>
<style scoped lang="scss">
  .abilities-list-item {
    gap: 8px;
    padding: 12px;
    border-radius: 6px;
    background: var(--side-panel);

    &.selected {
      background: var(--surface-3);
      outline: 2px solid var(--button-primary);
    }

    &__body {
      width: 100%;
    }

    &__label {
      gap: 8px;
      margin-bottom: 4px;

      @include flex(row, start, center);
      @include font-inter-500(14px, 10px, var(--text-primary));
    }

    &__text {
      @include line-clamp(2);
      @include font-inter-400(14px, 20px, var(--text-tertiary));
    }

    &__extra {
      @include font-inter-400(12px, 17px, var(--text-blind));
    }

    @include flex(row, start, space-between);
  }
</style>
