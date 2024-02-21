<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { FileIcon } from '~/shared/ui/icons'
  import { type File } from '../model'

  defineProps<{
    files: File[]
  }>()
</script>
<template>
  <div class="inline-files">
    <div
      v-for="file in files"
      :key="file.name"
      class="inline-files__item"
    >
      <FileIcon class="inline-files__icon" />
      <span class="inline-files__title">{{ file.name }}</span>
    </div>
  </div>
</template>
<style scoped lang="scss">
  .inline-files {
    &__item {
      margin-right: 4px;
      padding: 4px 6px;
      border-radius: 4px;
      background: var(--pill);

      @include flex(row, space-between, center);
    }

    &__title {
      display: block;
      margin-left: 3px;

      @include font-inter-500(12px, 14px, var(--text-secondary));
    }

    @include flex(row, space-between, space-between);
  }
</style>
