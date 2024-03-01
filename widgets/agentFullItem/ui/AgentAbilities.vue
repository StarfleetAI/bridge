<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { type Ability } from '~/entities/abilities'
  import { AbilityIcon } from '~/shared/ui/icons'

  defineProps<{
    abilities: Ability[]
  }>()
</script>
<template>
  <div class="agent-abilities">
    <div
      v-for="ability in abilities"
      :key="ability.id"
      class="agent-abilities__item"
    >
      <AbilityIcon />
      <div class="agent-abilities__title">
        {{ ability.name }}
      </div>
      <div class="agent-abilities__text">
        {{ ability.description }}
      </div>
    </div>
  </div>
</template>
<style scoped lang="scss">
  .agent-abilities {
    width: 100%;
    padding: 24px;

    &__item {
      width: 100%;
      margin-bottom: 6px;
      padding: 8px 12px;
      border-radius: 4px;
      background: var(--surface-3);

      @include flex(row, start, center, $gap: 12px);
    }

    &__title {
      @include flex(row, start, center);
      @include font-inter-500(14px, 20px, var(--text-secondary));
    }

    &__text {
      @include font-inter-400(14px, 20px, var(--text-tertiary));
    }
  }
</style>
