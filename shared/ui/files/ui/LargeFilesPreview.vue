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
    <div v-for="file in files" class="files-large-item">
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
  width: 30%;
  height: 120px;
  background: var(--surface-3);
  position: relative;
  border-radius: 12px;

  &__title {
    position: absolute;
    z-index: 2;
    top: 12px;
    left: 12px;

    @include font-inter-500(12px, 17px, var(--text-primary));
  }

  &__size {
    position: absolute;
    z-index: 2;
    bottom: 12px;
    left: 12px;

    @include font-inter-500(12px, 17px, var(--text-tertiary));
  }

  &__icon {
    z-index: 2;
    position: absolute;
    right: 12px;
    bottom: 8px;
  }

  &__frame {}

  &__type {
    position: absolute;
    z-index: 3;
    top: 8px;
    left: 6px;
  }

  &__type-title {
    position: absolute;
    z-index: 3;
    left: 0;
    width: 100%;
    bottom: 6px;
    text-align: center;

    @include font-inter-500(12px, 17px, var(--text-tertiary));
  }
}
</style>
