<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { highlight, languages } from 'prismjs'
  import 'prismjs/components/prism-python'
  import 'prismjs/themes/prism-tomorrow.css'
  import { PrismEditor } from 'vue-prism-editor'
  import 'vue-prism-editor/dist/prismeditor.min.css'

  const props = defineProps<{
    modelValue: string
    label: string
    readonly: boolean
  }>()

  const emits = defineEmits<{
    (event: 'update:modelValue', value: string): void
  }>()

  const code = ref(props.modelValue)

  watch(code, (val) => {
    if (!props.readonly) {
      emits('update:modelValue', val)
    }
  })

  const highlighter = (codeInput: string) => {
    return highlight(codeInput, languages.python, 'python')
  }
</script>

<template>
  <div class="code-input">
    <div
      v-if="label"
      class="code-input__label"
    >
      {{ label }}
    </div>
    <prism-editor
      v-model="code"
      :highlight="highlighter"
      :readonly="props.readonly"
      line-numbers
      :tab-size="4"
      class="font-mono text-sm code-input--editor"
    />
  </div>
</template>

<style scoped lang="scss">
  .code-input {
    overflow: hidden;
    border-radius: 8px;
    background: var(--surface-2);

    &__label {
      padding: 8px 12px;
      background: var(--surface-3);

      @include font-inter-500(14px, 20px, var(--text-primary));
      @include flex(row, space-between, center);
    }

    &--editor {
      padding: 8px 12px;
      background: var(--surface-2);
    }
  }
</style>
