<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { DocumentItem } from '~/entities/document'
</script>

<template>
  <div class="documents-list__date">March 2024</div>
  <DocumentItem />
  <DocumentItem />
  <DocumentItem />
  <div class="documents-list__date">February 2024</div>
  <DocumentItem />
  <DocumentItem />
  <DocumentItem />
</template>
<style scoped lang="scss">
  .documents-list {
    &__date {
      margin-bottom: 12px;

      @include font-inter-400(12px, 17px, var(--text-tertiary));
    }
  }
</style>
