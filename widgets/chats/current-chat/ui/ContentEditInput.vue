<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  const emits = defineEmits<{
    (e: 'submit'): void
  }>()
  const textModel = defineModel<string>()

  const { textarea, input } = useTextareaAutosize({ input: textModel })

  const handleKeyDown = (event: KeyboardEvent) => {
    const isShiftKey = event.shiftKey
    const isEnterKey = event.key === 'Enter'

    const moveToNextLine = () => {
      event.preventDefault()
      const eventTarget = event.target as HTMLInputElement
      const cursorPosition = eventTarget.selectionStart
      const before = textModel.value?.substring(0, cursorPosition!)
      const after = textModel.value?.substring(cursorPosition!)
      textModel.value = `${before}\n${after}`
    }

    if (!isShiftKey && isEnterKey && textModel.value) {
      event.preventDefault()
      emits('submit')
      return
    }
    if (isShiftKey && isEnterKey) {
      moveToNextLine()
    }
  }
</script>

<template>
  <div class="current-chat__input-container">
    <textarea
      ref="textarea"
      v-model="input"
      class="current-chat__input-text"
      placeholder="Message"
      @keydown="handleKeyDown"
    />
  </div>
</template>

<style lang="scss" scoped>
  .current-chat__input-container {
    @include flex($justify-content: center);
  }

  .current-chat__input-text {
    width: 100%;
    border: none;
    background-color: transparent;
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

    &:focus {
      outline: none;
    }

    @include hide-scrollbar;
    @include font-inter-400(16px, 22px, var(--text-secondary));
  }
</style>
