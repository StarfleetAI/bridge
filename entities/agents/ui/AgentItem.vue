<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { Switch } from '~/shared/ui/switch'
  import type { Agent } from '../model'

  defineProps<{
    agent: Agent
  }>()

  const active = ref(true)
</script>
<template>
  <div
    class="agents-list-item"
    :class="{ inactive: !active }"
  >
    <div class="agents-list-item__avatar" />
    <div class="agents-list-item__body">
      <div class="agents-list-item__label">
        {{ agent.name }}
      </div>
      <div class="agents-list-item__text">
        {{ agent.description }}
      </div>
      <Switch
        v-model="active"
        class="agents-list-item__switch"
      />
    </div>
  </div>
</template>
<style scoped lang="scss">
  .agents-list-item {
    position: relative;
    margin-bottom: 12px;
    padding: 12px 16px;
    border-radius: 6px;
    background: var(--side-panel);

    &.inactive {
      opacity: 0.5;
    }

    &__avatar {
      flex-shrink: 0;
      width: 80px;
      height: 80px;
      margin-right: 16px;
      border-radius: 50%;
      background: #d9d9d9;
    }

    &__label {
      gap: 8px;
      margin-bottom: 4px;

      @include flex(row, start, center);
      @include font-inter-700(16px, 22px, var(--text-primary));
    }

    &__text {
      padding-right: 40px;

      @include font-inter-400(14px, 20px, var(--text-tertiary));
    }

    &__switch {
      position: absolute !important;
      top: 15px;
      right: 16px;
    }

    @include flex(row, start, space-between);
  }
</style>
