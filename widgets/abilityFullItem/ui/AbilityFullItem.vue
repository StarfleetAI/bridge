<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { useAbilitiesStore, useAbilitiesNavigation } from '~/features/ability'
  import { type Ability } from '~/entities/abilities'
  import { CodeInput } from '~/shared/ui/code-input'

  const { selectedAbility } = useAbilitiesNavigation()
  const { getById } = useAbilitiesStore()

  const ability = ref(getById(selectedAbility.value!) as Ability)
</script>
<template>
  <div class="ability-full-item">
    <div class="ability-full-item__head">
      <div class="ability-full-item__title">
        <span>Ability</span>
      </div>
    </div>
    <div class="ability-full-item__body">
      <div class="ability-full-item__body-top">
        <div class="ability-full-item__name">
          <div>
            {{ ability.name }}
          </div>
          <div class="ability-full-item__info">by John F.</div>
        </div>
        <div class="ability-full-item__avatar" />
      </div>
      <div class="ability-full-item__result-text">
        {{ ability.description }}
      </div>
    </div>
    <div class="ability-full-item__bottom">
      <CodeInput
        v-model="ability.code"
        label="Code"
        readonly
      />
    </div>
  </div>
</template>
<style scoped lang="scss">
  .ability-full-item {
    &__head {
      height: 57px;
      padding: 12px 24px;
      border-bottom: 1px solid var(--border-3);

      @include flex(row, space-between, center);
    }

    &__title {
      @include font-inter-700(14px, 20px, var(--text-secondary));
    }

    &__name {
      gap: 8px;

      & > div {
        gap: 8px;

        @include flex(row, start, center);
      }

      @include flex(column, start, start);
      @include font-inter-700(20px, 28px, var(--text-primary));
    }

    &__info {
      @include font-inter-500(14px, 20px, var(--text-tertiary));
    }

    &__body {
      padding: 24px;
      border-bottom: 0.5px solid var(--pill);
    }

    &__body-top {
      margin-bottom: 16px;

      @include flex(row, space-between, center);
    }

    &__result-text {
      position: relative;

      @include font-inter-400(14px, 19px, var(--text-secondary));
    }

    &__bottom {
      padding: 24px;
    }
  }
</style>
