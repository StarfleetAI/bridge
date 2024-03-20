<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { AttachmentIcon, SendIcon } from '~/shared/ui/icons'

  const props = withDefaults(
    defineProps<{
      isProcessing: boolean
      withFiles?: boolean
      singleLine?: boolean
    }>(),
    {
      withFiles: true,
    },
  )
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
  <div class="chat-input-container">
    <div class="chat-input">
      <textarea
        ref="textarea"
        v-model.trim="input"
        :class="['chat-input-text', { 'full-width': !withFiles }]"
        placeholder="Message"
        @keydown="handleKeyDown"
      />
      <SendIcon
        v-if="!isProcessing"
        class="chat-input-send"
        @click="$emit('submit')"
      />

      <template v-if="withFiles">
        <label
          for="currrent-chat__input-file"
          class="chat-input-file-icon"
        >
          <AttachmentIcon />
        </label>
        <input
          id="currrent-chat__input-file"
          type="file"
          class="chat-input-file"
          @input="handleFileInput"
        />
      </template>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .chat-input-container {
    align-self: center;
    width: 100%;

    @include flex($justify-content: center);
  }

  .chat-input {
    position: relative;
    gap: 7px;
    width: 100%;
    max-width: 680px;

    @include flex(column);
  }

  .chat-input-text {
    width: 100%;
    min-height: 48px;
    max-height: 200px;
    padding: 16px 52px;
    border: 0.5px solid var(--text-tertiary);
    border-radius: 12px;
    background-color: transparent;
    resize: none;
    transition: border-color 0.1s ease;

    &:focus {
      border-width: 1px;
      border-color: var(--text-secondary);
      color: var(--text-primary);
      outline: none;
    }

    &::placeholder {
      @include font-inter-400(16px, 22px, var(--text-tertiary));
    }

    &.full-width {
      padding-left: 16px;
    }

    @include hide-scrollbar;
    @include font-inter-500(16px, 22px, var(--text-secondary));
  }

  .chat-input-file {
    position: absolute;
    right: 0;
    display: none;
  }

  .chat-input-file-icon {
    position: absolute;
    top: 16px;
    left: 16px;
    color: var(--text-tertiary);

    &:hover {
      color: var(--text-secondary);
    }
  }

  .chat-input-send {
    position: absolute;
    top: 16px;
    right: 16px;
    color: var(--text-tertiary);

    &:hover {
      color: var(--text-secondary);
    }
  }
</style>
