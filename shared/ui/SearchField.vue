<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { CrossIcon, SearchIcon } from './icons'

  defineProps<{
    placeholder?: string
  }>()
  const model = defineModel<string>({ required: true })

  const isFilled = computed(() => model.value.length > 0)
  const resetModel = () => (model.value = '')

  const isInFocus = ref(false)
</script>

<template>
  <div
    :class="['search-field', { filled: isFilled, focused: isInFocus }]"
    @focusin="isInFocus = true"
    @focusout="isInFocus = false"
  >
    <SearchIcon
      width="20"
      height="20"
      class="search-field__icon"
    />
    <input
      v-model="model"
      type="text"
      :placeholder="placeholder"
      :class="['search-field__input', { filled: isFilled }]"
    />
    <CrossIcon
      v-if="isFilled"
      width="20"
      height="20"
      class="search-field__icon reset"
      @click="resetModel"
    />
  </div>
</template>

<style lang="scss" scoped>
  .search-field {
    position: relative;
    width: 100%;
    padding: 6px 0;
    border: 2px solid transparent;
    border-radius: 6px;
    background-color: transparent;

    &.filled {
      background-color: var(--surface-2);
    }

    &.focused {
      border-color: var(--button-primary);
      background-color: var(--surface-2);
    }

    &:hover {
      &:not(.filled, .focused) .search-field__input {
        color: var(--text-secondary);

        &::placeholder {
          color: var(--text-secondary);
        }
      }
    }
  }

  .search-field__icon {
    position: absolute;
    top: 10px;
    left: 8px;
    color: var(--text-tertiary);

    &.reset {
      right: 8px;
      left: initial;
    }
  }

  .search-field__input {
    display: flex;
    width: 100%;
    height: 28px;
    padding: 0 6px 0 36px;
    background-color: transparent;
    outline: none;

    &.filled {
      padding-right: 34px;
      color: var(--text-primary);
    }

    &::placeholder {
      @include font-inter-400(14px, 20px, var(--text-tertiary));
    }

    @include font-inter-400(14px, 20px, var(--text-tertiary));
  }
</style>
