<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { useAbilitiesStore, useAbilitiesNavigation } from '~/features/ability'
  import { type Ability } from '~/entities/abilities'
  import { BaseButton } from '~/shared/ui/base'
  import { CodeInput } from '~/shared/ui/code-input'
  import { EditIcon } from '~/shared/ui/icons'

  const { selectedAbility, enableEditAbility } = useAbilitiesNavigation()
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
          {{ ability.name }}
        </div>
        <div class="ability-full-item__avatar" />
      </div>
      <div class="ability-full-item__result-text">
        {{ ability.description }}
      </div>
      <div class="ability-full-item__info">by John F.</div>
      <BaseButton
        size="large"
        color="gray"
        class="agent-full-item__button install"
        @click="enableEditAbility()"
      >
        <template #icon>
          <EditIcon />
        </template>
        Edit
      </BaseButton>
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
      margin-bottom: 8px;

      @include font-inter-500(14px, 20px, var(--text-tertiary));
    }

    &__body {
      padding: 24px;
      border-bottom: 0.5px solid var(--pill);
    }

    &__body-top {
      margin-bottom: 8px;

      @include flex(row, space-between, center);
    }

    &__result-text {
      position: relative;
      margin-bottom: 8px;

      @include font-inter-400(14px, 19px, var(--text-secondary));
    }

    &__bottom {
      padding: 24px;
    }
  }
</style>
