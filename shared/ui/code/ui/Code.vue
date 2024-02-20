<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { highlight, languages } from 'prismjs'
  import 'prismjs/components/prism-python'
  import 'prismjs/themes/prism-tomorrow.css'
  import { PrismEditor } from 'vue-prism-editor'

  import 'vue-prism-editor/dist/prismeditor.min.css'

  const highlighter = (code: string) => {
    return highlight(code, languages.python, 'python')
  }

  const req = ref({
    name: '',
    description: '',
    code: `def do_something(
    arg1: Annotated[str, "String argument"],
    arg2: Annotated[int, "Integer argument"]
) -> str:
    # Do the actual job here
    return "Something was successful!"`,
  })
</script>

<template>
  <div class="code-block">
    <div class="code-block__title">Code</div>
    <prism-editor
      v-model="req.code"
      :highlight="highlighter"
      line-numbers
      :tab-size="4"
      class="bg-gray-800 rounded-lg px-3 py-5 font-mono text-sm"
    />
  </div>
</template>

<style scoped lang="scss">
  .code-block {
    overflow: hidden;
    border-radius: 8px;
    background: var(--surface-3);

    &__title {
      padding: 8px 12px;
      background: var(--surface-5);

      @include font-inter-500(14px, 20px, var(--text-primary));
      @include flex(row, space-between, center);
    }
  }
</style>
