<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { AttachmentIcon, SendIcon } from '~/shared/ui/icons'

  const props = defineProps<{
    isProcessing: boolean
  }>()
  const emits = defineEmits<{
    (e: 'submit'): void
  }>()
  const textModel = defineModel<string>()
  const fileModel = defineModel<File>('file')

  const { textarea, input } = useTextareaAutosize({ input: textModel })
  const handleFileInput = (event: Event) => {
    const target = event.target as HTMLInputElement
    const file = target.files?.[0]
    fileModel.value = file
  }

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

    if (!isShiftKey && isEnterKey && input.value) {
      if (props.isProcessing) {
        moveToNextLine()
        return
      }
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
    <div class="current-chat__input">
      <textarea
        ref="textarea"
        v-model.trim="input"
        class="current-chat__input-text"
        placeholder="Message"
        @keydown="handleKeyDown"
      />
      <SendIcon
        v-if="!isProcessing"
        class="current-chat__input-send"
        @click="$emit('submit')"
      />

      <label
        for="currrent-chat__input-file"
        class="current-chat__input-file-icon"
      >
        <AttachmentIcon />
      </label>
      <input
        id="currrent-chat__input-file"
        type="file"
        class="current-chat__input-file"
        @input="handleFileInput"
      />
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .current-chat__input-container {
    align-self: center;
    width: 100%;
    margin-top: auto;
    margin-bottom: 32px;
    padding: 0 24px;

    @include flex($justify-content: center);
  }

  .current-chat__input {
    position: relative;
    gap: 7px;
    width: 100%;
    max-width: 680px;

    @include flex(column);
  }

  .current-chat__input-text {
    width: 100%;
    min-height: 56px;
    max-height: 200px;
    padding: 16px 52px;
    border: 1px solid var(--text-tertiary);
    border-radius: 8px;
    background-color: var(--surface-1);
    resize: none;
    transition: border-color 0.1s ease;

    &:focus {
      border-color: var(--border-standart);
      outline: none;
    }

    @include hide-scrollbar;
    @include font-inter-500(16px, 22px, var(--text-primary));
  }

  .current-chat__input-file {
    position: absolute;
    right: 0;
    display: none;
  }

  .current-chat__input-file-icon {
    position: absolute;
    top: 16px;
    left: 16px;
    color: var(--text-tertiary);

    &:hover {
      color: var(--text-secondary);
    }
  }

  .current-chat__input-send {
    position: absolute;
    top: 16px;
    right: 16px;
    color: var(--text-tertiary);

    &:hover {
      color: var(--text-secondary);
    }
  }
</style>
