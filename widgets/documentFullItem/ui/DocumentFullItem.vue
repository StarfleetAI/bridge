<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  // import { TaskItem } from '~/entities/tasks'
  import { DocumentTitleIcon, TaskHeaderIcon, FileFrameIcon, FileCSVIcon } from '~/shared/ui/icons'
</script>
<template>
  <div class="document-full-item">
    <div class="document-full-item__head">
      <div class="document-full-item__title">
        <DocumentTitleIcon />
        <span>Document #0393</span>
      </div>
    </div>
    <div class="document-full-item__body">
      <div class="document-full-item__body-top">
        <div class="document-full-item__name">
          <div>list012.csv</div>
          <div class="document-full-item__info">by John F. • installed 54,322 times</div>
          <div class="document-full-item__info">878kb</div>
        </div>
        <div class="document-full-item__icon">
          <FileFrameIcon class="document-full-item__frame" />
          <FileCSVIcon class="document-full-item__type" />
          <div class="document-full-item__type-title">CSV</div>
        </div>
      </div>
      <div class="document-full-item__abilities-title">
        <TaskHeaderIcon />
        Associated Tasks
      </div>
      <!-- <TaskItem class="document-full-item__task" />
      <TaskItem class="document-full-item__task" />
      <TaskItem class="document-full-item__task" /> -->
    </div>
  </div>
</template>
<style scoped lang="scss">
  .document-full-item {
    &__head {
      padding: 12px 24px;
      border-bottom: 0.5px solid var(--pill);
    }

    &__title {
      gap: 4px;

      @include flex(row, start, center);
      @include font-inter-500(14px, 22px, var(--text-tertiary));
    }

    &__name {
      gap: 4px;
      margin-bottom: 32px;

      @include flex(column, start, start);
      @include font-inter-700(20px, 28px, var(--text-primary));
    }

    &__info {
      @include font-inter-500(14px, 20px, var(--text-tertiary));
    }

    &__body {
      padding: 24px 24px 0;
    }

    &__body-top {
      @include flex(row, space-between, center);
    }

    &__abilities {
      padding: 24px;
    }

    &__abilities-title {
      gap: 5px;
      margin-bottom: 16px;

      @include flex(row, start, center);
      @include font-inter-500(14px, 20px, var(--text-tertiary));
    }

    &__task {
      background: var(--surface-3);
    }

    &__icon {
      position: relative;
      bottom: 8px;
      z-index: 2;
    }

    &__type {
      position: absolute;
      top: 8px;
      left: 6px;
      z-index: 3;
    }

    &__type-title {
      position: absolute;
      bottom: 6px;
      left: 0;
      z-index: 3;
      width: 100%;
      text-align: center;

      @include font-inter-700(10px, 14px, var(--text-tertiary));
    }
  }
</style>
