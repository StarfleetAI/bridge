<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { BaseDropdown, BaseDropdownItem } from '~/shared/ui/dropdown'
  import { DropdownIcon } from '~/shared/ui/icons'
  import type { Model } from '../model'

  defineProps<{ models: Model[] }>()

  const modelValue = defineModel<Nullable<string>>()

  const setModel = (model: string) => {
    if (model === 'default') {
      modelValue.value = null
    } else {
      modelValue.value = model
    }
  }
</script>

<template>
  <BaseDropdown placement="bottom-start">
    <div class="model-selected">
      <span>
        {{ modelValue || 'Default' }}
      </span>
      <DropdownIcon />
    </div>
    <template #content>
      <BaseDropdownItem
        v-close-popper
        class="model-dropdown__item"
        @click="setModel('default')"
      >
        <template #label>Default </template>
      </BaseDropdownItem>
      <BaseDropdownItem
        v-for="model in models"
        :key="model.name"
        v-close-popper
        class="model-dropdown__item"
        @click="setModel(`${model.provider}/${model.name}`)"
      >
        <template #label>{{ `${model.provider}/${model.name}` }}</template>
      </BaseDropdownItem>
    </template>
  </BaseDropdown>
</template>

<style lang="scss" scoped>
  .model-selected {
    flex: 1;
    width: 100%;
    padding: 8px 12px;
    border: 1px solid var(--border-2);
    border-radius: 6px;

    & span {
      @include text-ellipsis;
    }

    @include flex(row, space-between, center, 6px);
    @include font-inter-400(14px, 20px, var(--text-secondary));
  }

  .model-dropdown__item {
    &:hover {
      background-color: var(--surface-4);
    }

    @include font-inter-500(16px, 22px, var(--text-primary));
    @include text-ellipsis;
  }
</style>
