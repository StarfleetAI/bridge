<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { AvatarsList } from '~/shared/ui/avatars'
</script>
<template>
  <div class="document-full-item">
    <div class="document-full-item__head">
      <div class="document-full-item__title">
        <span>Document</span>
      </div>
    </div>
    <div class="document-full-item__body">
      <div class="document-full-item__body-top">
        <div class="document-full-item__info">878kb • 12.09.24, 19:43</div>
        <AvatarsList :agents="[]" />
      </div>
      <div class="document-full-item__name">
        <div>list012.csv</div>
      </div>
    </div>
  </div>
</template>
<style scoped lang="scss">
  .document-full-item {
    &__head {
      height: 57px;
      padding: 12px 24px;
      border-bottom: 1px solid var(--border-3);

      @include flex(row, space-between, center);
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
      @include font-inter-500(18px, 25px, var(--text-primary));
    }

    &__info {
      @include font-inter-500(14px, 20px, var(--text-tertiary));
    }

    &__body {
      padding: 24px 24px 0;
    }

    &__body-top {
      margin-bottom: 10px;

      @include flex(row, space-between, center);
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
