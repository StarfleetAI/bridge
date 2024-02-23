<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { formatBytes } from '~/shared/lib'
  import { FileIcon, UnlinkIcon } from '~/shared/ui/icons'
  // import { type File } from '../model'
  defineProps<{
    files: File[]
  }>()
  defineEmits<{
    (e: 'remove', file: File): void
  }>()
</script>
<template>
  <div class="files-list-items">
    <div
      v-for="file in files"
      :key="file?.name"
      class="files-list-item"
    >
      <div class="files-list-item__icon">
        <FileIcon />
      </div>
      <div class="file__name">{{ file?.name }}</div>
      <div class="file__size">{{ formatBytes(file?.size) }}</div>
      <!-- <div>{{ file?.rows }} rows</div> -->
      <div class="file__date">{{ $dayjs(file?.lastModified).format('DD.MM.YY, HH:mm') }}</div>
      <div
        class="file__remove"
        @click="$emit('remove', file)"
      >
        <UnlinkIcon />
      </div>
    </div>
  </div>
</template>
<style scoped lang="scss">
  .files-list-items {
    padding: 8px 0;
  }

  .files-list-item {
    position: relative;
    display: grid;
    grid-template-columns: auto minmax(100px, 250px) 1fr 1fr;
    gap: 8px;
    align-items: center;
    overflow: hidden;
    width: 100%;
    margin-bottom: 4px;
    padding-right: 12px;
    border-radius: 4px;
    background-color: var(--surface-3);

    &__icon {
      width: 40px;
      height: 40px;
      background: var(--surface-4);

      @include flex(row, center, center);
    }

    &:hover {
      .file__remove {
        display: flex;
      }
    }

    @include font-inter-500(12px, 17px, var(--text-secondary));
  }

  .file__name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file__size {
    white-space: nowrap;
  }

  .file__date {
    white-space: nowrap;
  }

  .file__remove {
    position: absolute;
    right: 12px;
    display: none;
    justify-content: flex-end;
    align-items: center;
    width: 68px;
    height: 100%;
    background: linear-gradient(270deg, #2a2f3d 67.5%, rgba(42, 47, 61, 0) 100%);
  }
</style>
