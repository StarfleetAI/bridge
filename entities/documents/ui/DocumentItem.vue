<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { DocumentCheck } from '~/shared/ui/icons'

  import type { Document } from '../model'

  defineProps<{
    document: Document
  }>()
</script>
<template>
  <div class="document-item">
    <div class="document-item__file">
      <div class="document-item__preview"></div>
      <div class="document-item__title">{{ document?.title }}</div>
    </div>
    <div class="document-item__owner" />
    <div class="document-item__info">
      <div class="document-item__check"><DocumentCheck /> 5</div>
      <div class="document-item__size">872kb</div>
      <div class="document-item__date">12.09.24, 19:43</div>
    </div>
  </div>
</template>
<style scoped lang="scss">
  .document-item {
    overflow: hidden;
    height: 45px;
    margin-bottom: 12px;
    border-radius: 4px;
    background: var(--surface-3);

    &.selected {
      background: var(--surface-3);
      outline: 2px solid var(--button-primary);
    }

    &__file {
      width: 30%;

      @include flex(row, start, center);
    }

    &__preview {
      width: 40px;
      height: 40px;
      margin-right: 8px;

      @include flex(row, center, center);
    }

    &__title {
      @include font-inter-500(14px, 19px, var(--text-primary));
    }

    &__owner {
      width: 30%;
    }

    &__info {
      width: 40%;
      padding-right: 16px;

      @include flex(row, end, center);
    }

    &__check {
      gap: 2px;

      @include flex(row, start, center);
    }

    &__size {
      margin-left: 16px;
    }

    &__date {
      margin-left: 16px;
    }

    @include flex(row, start, center);
    @include font-inter-500(12px, 17px, var(--text-tertiary));
  }
</style>
