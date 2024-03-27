<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  defineProps<{
    placeholder?: string
  }>()

  const modelValue = defineModel<string>()

  const inputRef = ref<HTMLInputElement>()
  const focus = () => {
    inputRef.value?.focus()
  }

  defineExpose({ focus })
</script>

<template>
  <input
    ref="inputRef"
    v-model="modelValue"
    class="task-form__input"
    :placeholder="placeholder"
  />
</template>

<style lang="scss" scoped>
  @import './task-input';
</style>
