<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useDocumentsNavigation } from '~/features/document'
  import { DocumentItem, type Document } from '~/entities/documents'

  defineProps<{
    documents: Document[]
  }>()
  const { setSelectedDocument, selectedDocument } = useDocumentsNavigation()
</script>

<template>
  <div class="documents-list__date">March 2024</div>
  <div class="documents-list">
    <DocumentItem
      v-for="document in documents"
      :key="document.id"
      :document="document"
      :class="{ selected: document.id === selectedDocument }"
      @click="setSelectedDocument(document.id)"
    />
  </div>
</template>
<style scoped lang="scss">
  .documents-list {
    padding: 16px 24px;

    &__date {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
      gap: 12px;
      margin-bottom: 12px;
      padding: 16px 24px;

      @include font-inter-400(12px, 17px, var(--text-tertiary));
    }
  }
</style>
