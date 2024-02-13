<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { AttachmentIcon } from '~/shared/ui/icons'

  const emits = defineEmits<{
    (e: 'submit'): void
  }>()
  const textModel = defineModel<string>()
  const fileModel = defineModel<File>('file')

  const textarea = ref<HTMLTextAreaElement>()

  //   const autoResize = () => {
  //     if (textarea.value) {
  //       textarea.value.style.height = 'auto'
  //       textarea.value.style.height = `${textarea.value.scrollHeight}px`
  //     }
  //   }
  const handleFileInput = (event: Event) => {
    const target = event.target as HTMLInputElement
    const file = target.files?.[0]
    fileModel.value = file
  }

  const handleKeyDown = (event: KeyboardEvent) => {
    const isCmdOrCtrl = event.metaKey || event.ctrlKey // Cmd for Mac, Ctrl for Windows/Linux
    const isAlt = event.altKey
    if (!isCmdOrCtrl && !isAlt && event.key === 'Enter' && textModel.value) {
      event.preventDefault()
      emits('submit')
      return
    }
    if ((isCmdOrCtrl || isAlt) && event.key === 'Enter') {
      event.preventDefault() // Prevent the default action
      const eventTarget = event.target as HTMLInputElement
      // Calculate the new position and insert the newline
      const cursorPosition = eventTarget.selectionStart
      const before = textModel.value?.substring(0, cursorPosition!)
      const after = textModel.value?.substring(cursorPosition!)
      textModel.value = `${before}\n${after}`
    }
  }
</script>

<template>
  <div class="current-chat__input">
    <textarea
      ref="textarea"
      v-model="textModel"
      class="current-chat__input-text"
      placeholder="Message"
      @keydown="handleKeyDown"
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
    <div class="current-chat__input-help"><b>Enter</b> to send</div>
  </div>
</template>

<style lang="scss" scoped>
  .current-chat__input {
    position: relative;
    gap: 7px;
    align-self: center;
    width: calc(100% - 96px);
    margin-top: auto;
    margin-bottom: 56px;

    @include flex(column);
  }

  .current-chat__input-text {
    width: 100%;
    height: 86px;
    max-height: 200px;
    padding: 16px 52px 16px 16px;
    border: 1px solid var(--text-tertiary);
    border-radius: 8px;
    background-color: var(--surface-1);
    resize: none;

    &:focus {
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
    right: 16px;
    color: var(--text-tertiary);
  }

  .current-chat__input-help {
    width: 100%;
    text-align: center;

    & b {
      color: var(--text-tertiary);
    }

    @include font-inter-400(12px, 17px, var(--text-tertiary));
  }
</style>
