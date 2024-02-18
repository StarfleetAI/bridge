<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { FileIcon } from '~/shared/icons'
  import { type File } from '../model'

  defineProps<{
    files: File[]
  }>()
</script>
<template>
  <div class="files-list-items">
    <div
      v-for="file in files"
      class="files-list-item"
    >
      <div class="files-list-item__icon">
        <FileIcon />
      </div>
      <div>{{ file?.name }}</div>
      <div>{{ file?.size }}</div>
      <div>{{ file?.rows }} rows</div>
      <div>{{ file?.created }}</div>
    </div>
  </div>
</template>
<style scoped lang="scss">
  .files-list-items {
    padding: 8px 0;
  }

  .files-list-item {
    display: grid;
    grid-template-columns: auto 100px 1fr 1fr 1fr;
    gap: 8px;
    align-items: center;
    overflow: hidden;
    width: 100%;
    margin-bottom: 4px;
    border-radius: 4px;

    &__icon {
      width: 40px;
      height: 40px;
      background: var(--surface-4);

      @include flex(row, center, center);
    }

    @include font-inter-500(12px, 17px, var(--text-secondary));
  }
</style>
