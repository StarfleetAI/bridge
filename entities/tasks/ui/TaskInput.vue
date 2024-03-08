<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  defineProps<{
    isSummary?: boolean
    placeholder?: string
  }>()

  const modelValue = defineModel<string>()

  const { textarea, input } = useTextareaAutosize({ input: modelValue })

  const focus = () => {
    textarea.value?.focus()
  }

  defineExpose({ focus })
</script>

<template>
  <textarea
    ref="textarea"
    v-model="input"
    :class="['task-form__input', { summary: isSummary }]"
    :placeholder="placeholder"
  />
</template>

<style lang="scss" scoped>
  .task-form__input {
    min-height: 41px;
    max-height: 66px;
    padding: 8px 12px;
    border: none;
    border-radius: 6px;
    background-color: transparent;
    outline: none;
    font-family: inherit;
    resize: none;
    font-feature-settings: inherit;
    font-variation-settings: inherit;

    &:before,
    &:after {
      box-sizing: border-box;
      border-width: 0;
      border-style: solid;
      border-color: transparent;
    }

    &.summary {
      min-height: 20px;
      max-height: 136px;

      @include font-inter-500(14px, 20px, var(--text-primary));
    }

    &:focus {
      background-color: var(--surface-3);
    }

    @include hide-scrollbar;
    @include font-inter-500(18px, 25px, var(--text-primary));
  }
</style>
