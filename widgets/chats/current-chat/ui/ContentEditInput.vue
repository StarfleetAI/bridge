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
      event.preventDefault() // Prevent the default action
      const eventTarget = event.target as HTMLInputElement
      // Calculate the new position and insert the newline
      const cursorPosition = eventTarget.selectionStart!
      const before = eventTarget.value?.substring(0, cursorPosition)
      const after = eventTarget.value?.substring(cursorPosition)
      const newValue = `${before}\n${after}`

      eventTarget.value = newValue

      const newCursorPosition = cursorPosition + 1
      eventTarget.setSelectionRange(newCursorPosition, newCursorPosition)
      nextTick(() => (input.value = newValue))
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
    @include flex($justify: center);
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
