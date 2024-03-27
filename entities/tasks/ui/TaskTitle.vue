<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  const props = defineProps<{
    placeholder?: string
    currentTitle?: string
    taskId?: number
  }>()
  const emits = defineEmits<{
    save: []
  }>()
  const modelValue = defineModel<string>()

  const inputRef = ref<HTMLInputElement>()
  const focus = () => {
    inputRef.value?.focus()
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
    if (modelValue.value) {
      emits('save')
      inputRef.value?.blur()
      isEditing.value = false
    } else {
      resetChanges()
    }
  }

  const taskTitlePlaceholder = computed(() => {
    if (props.currentTitle) {
      return props.currentTitle
    } else if (props.taskId) {
      return `Task #${props.taskId}`
    }
    return 'Title'
  })

  const resetChanges = () => {
    blur()
    isEditing.value = false
    modelValue.value = toRaw(props.currentTitle)
  }
</script>

<template>
  <div :class="['task-input__container', { 'task-input__container--editing': isEditing }]">
    <input
      ref="inputRef"
      v-model="modelValue"
      :class="['task-input', { 'task-input--editing': isEditing }]"
      :title="taskTitlePlaceholder"
      placeholder="Title"
      @blur="save"
      @click="enableIsEditing"
      @keydown.esc="resetChanges"
      @keydown.enter="save"
    />
  </div>
</template>

<style lang="scss" scoped>
  @import './task-input';
</style>
