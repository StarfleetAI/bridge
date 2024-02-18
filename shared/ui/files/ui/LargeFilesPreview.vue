<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { FileCSVIcon, FileFrameIcon } from '~/shared/icons'
  import { type File } from '../model'

  defineProps<{
    files: File[]
  }>()
</script>
<template>
  <div class="files-large-items">
    <div
      v-for="file in files"
      class="files-large-item"
    >
      <div class="files-large-item__title">
        {{ file?.name }}
      </div>
      <div class="files-large-item__size">
        {{ file?.size }}
      </div>
      <div class="files-large-item__icon">
        <FileFrameIcon class="files-large-item__frame" />
        <FileCSVIcon class="files-large-item__type" />
        <div class="files-large-item__type-title">CSV</div>
      </div>
    </div>
  </div>
</template>
<style scoped lang="scss">
  .files-large-items {
    flex-wrap: wrap;
    gap: 8px;
    margin-bottom: 24px;

    @include flex(row, space-between, center);
  }

  .files-large-item {
    position: relative;
    width: 30%;
    height: 120px;
    border-radius: 12px;
    background: var(--surface-3);

    &__title {
      position: absolute;
      top: 12px;
      left: 12px;
      z-index: 2;

      @include font-inter-500(12px, 17px, var(--text-primary));
    }

    &__size {
      position: absolute;
      bottom: 12px;
      left: 12px;
      z-index: 2;

      @include font-inter-500(12px, 17px, var(--text-tertiary));
    }

    &__icon {
      position: absolute;
      right: 12px;
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
