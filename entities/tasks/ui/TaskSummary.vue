<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { ResizeIcon } from '~/shared/ui/icons'

  const props = defineProps<{
    placeholder?: string
    currentSummary?: string
  }>()
  const emits = defineEmits<{
    save: []
  }>()
  const modelValue = defineModel<string>()

  const { textarea, input } = useTextareaAutosize({ input: modelValue })

  const focus = () => {
    textarea.value?.focus()
  }
  const blur = () => {
    textarea.value?.blur()
  }

  defineExpose({ focus })

  const isEditing = ref(false)
  const enableIsEditing = () => {
    if (isEditing.value) {
      return
    }
    isEditing.value = true
  }
  const save = async () => {
    emits('save')
    isEditing.value = false
  }

  const isFull = ref(false)
  const toggleIsFull = () => {
    isFull.value = !isFull.value
  }
  const rows = computed(() => {
    if (isEditing.value) {
      return undefined
    }
    return 6
  })
  const { height } = useElementSize(textarea)

  const showResizeButton = computed(() => {
    return height.value > 100
  })
  watch(isFull, (newVal) => {
    if (newVal) {
      textarea.value.style.maxHeight = textarea.value.scrollHeight + 'px'
      setTimeout(() => {
        textarea.value.style.maxHeight = 'unset'
      }, 200)
    } else {
      textarea.value.style.maxHeight = '128px'
    }
  })

  const resetChanges = () => {
    blur()
    isEditing.value = false
    modelValue.value = toRaw(props.currentSummary)
  }

  /**  Save on press Shift + Cmd/Ctrl + Enter */
  const handleKeydown = (event: KeyboardEvent) => {
    if (event.shiftKey && (event.metaKey || event.ctrlKey) && event.key === 'Enter') {
      save()
      blur()
    }
  }
</script>

<template>
  <div :class="['task-input__container summary', { 'task-input__container--editing': isEditing }]">
    <textarea
      ref="textarea"
      v-model="input"
      :class="['task-input summary', { full: isFull }]"
      placeholder="No summary"
      :rows="rows"
      @click="enableIsEditing"
      @blur="save"
      @keydown.esc="resetChanges"
      @keydown="handleKeydown"
    >
    </textarea>

    <div
      v-if="showResizeButton"
      class="title-resize"
    >
      <ResizeIcon
        color="var(--text-tertiary)"
        @click="toggleIsFull"
      />
    </div>
  </div>
</template>

<style lang="scss" scoped>
  @import './task-input';

  .task-input__container {
    display: block;
  }

  .title-resize {
    & svg {
      margin-right: 12px;
      margin-bottom: 8px;
    }

    @include flex(row, flex-end);
  }
</style>
