<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  defineProps<{
    name: string
    description: string
  }>()
  const model = defineModel<string>()
</script>

<template>
  <div class="settings-field">
    <div class="settings-field__info">
      <div class="settings-field__name">{{ name }}</div>
      <div class="settings-field__description">{{ description }}</div>
    </div>
    <input
      v-model="model"
      type="text"
      class="settings-field__input"
    />
  </div>
</template>

<style lang="scss" scoped>
  .settings-field {
    width: 100%;
    height: 40px;

    @include flex($gap: 16px);
  }

  .settings-field__info {
    flex-shrink: 0;
    width: 182px;
    text-align: end;

    @include flex(column, $align-items: flex-end, $gap: 4px);
  }

  .settings-field__name {
    @include font-inter-700(12px, 17px, var(--text-secondary));
  }

  .settings-field__description {
    word-break: break-word;

    @include font-inter-400(12px, 17px, var(--text-tertiary));
  }

  .settings-field__input {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid var(--border-2);
    border-radius: 6px;
    background-color: var(--surface-2);
    outline: none;

    &:focus {
      border-color: var(--text-secondary);
      color: var(--text-secondary);
    }

    @include font-inter-400(14px, 20px, var(--text-tertiary));
  }
</style>
