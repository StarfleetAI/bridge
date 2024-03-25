<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { TaskResultKind, type TaskResult } from '~/entities/tasks'
  import { highlightCode } from '~/shared/lib'

  const props = defineProps<{
    result: TaskResult
  }>()

  const resultRef = ref<HTMLDivElement>()

  const parseAndHighlightContent = () => {
    if (resultRef.value) {
      highlightCode(resultRef.value)
    }
  }
  onMounted(() => {
    parseAndHighlightContent()
  })
  watch(
    () => props.result,
    async () => {
      await nextTick()
      parseAndHighlightContent()
    },
    {
      deep: true,
    },
  )
  const resultContent = computed(() => {
    if (props.result.kind === TaskResultKind.Text) {
      return props.result.parsed_data || ''
    }
    return props.result.data
  })
</script>

<template>
  <div class="result">
    <div class="result__body">
      <div class="result__content">
        <div
          ref="resultRef"
          class="result__content-markdown"
          v-html="resultContent"
        />
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .result {
    position: relative;
    z-index: 2;
    gap: 8px;

    @include flex(row, flex-start, stretch);
  }

  .result__body {
    flex: 1 0;
    gap: 8px;
    width: 100%;
    min-width: 0;

    @include flex(column, flex-start, stretch);
  }

  .result__content {
    border-radius: 6px;

    @include font-inter-400(16px, 22px, var(--text-primary));
    @include flex(column, $gap: 16px);
  }

  .result__content-markdown {
    cursor: auto;
    user-select: initial;

    & pre {
      white-space: pre-wrap;
    }

    @include flex(column, flex-start, flex-start, 16px);
  }

  :deep(.hljs-copy-wrapper) {
    position: relative;
    overflow: hidden;
    width: 100%;
    min-width: 0;
    max-width: 688px;
    border-radius: 6px;

    &:before {
      content: attr(data-language);
      order: 1;
      width: 100%;
      padding: 8px 12px;
      background-color: var(--surface-5);
      font-family: Inter, sans-serif;

      @include font-inter-500(14px, 20px, var(--text-primary));
    }

    @include flex(column-reverse);
  }

  :deep(.hljs-copy-button) {
    position: absolute;
    top: 8px;
    right: 12px;
    display: flex;
    gap: 4px;
    justify-content: flex-end;
    align-items: center;
    align-self: flex-end;
    width: auto;
    min-width: 52px;
    padding-left: 16px;
    border: none;
    background-color: transparent;
    font-family: Inter, sans-serif;
    text-align: end;
    cursor: default;

    &:before {
      content: '';
      width: 16px;
      height: 16px;
      background: transparent url('~/assets/svg/copy-icon.svg') no-repeat left;
    }

    @include font-inter-500(14px, 20px, var(--text-secondary));
  }

  :deep(.hljs-copy-alert) {
    display: none;
  }

  :deep(code) {
    overflow: auto;
    overflow-y: hidden;
    overscroll-behavior: auto;

    @include add-scrollbar;
  }

  :deep(code[data-highlighted='yes']) {
    background-color: var(--surface-3);
  }

  :deep(pre code) {
    white-space: pre-wrap;
  }

  :deep(pre code.hljs) {
    padding: 8px 12px;
    white-space: pre;
  }
</style>
