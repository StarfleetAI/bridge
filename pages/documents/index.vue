<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { DocumentsList } from '~/widgets/documentsList'
  import { useDocumentsNavigation, useDocumentsStore } from '~/features/document'
  import { BaseContainer, BaseButton } from '~/shared/ui/base'
  import { PlusIcon } from '~/shared/ui/icons'

  definePageMeta({
    title: 'Documents',
  })

  const { documents } = storeToRefs(useDocumentsStore())

  const { isCreateDocument, enableCreateDocument, selectedDocument } = useDocumentsNavigation()

  const DocumentFullItem = defineAsyncComponent(async () => {
    const module = await import('~/widgets/documentFullItem')
    return module.DocumentFullItem
  })

  const DocumentForm = defineAsyncComponent(async () => {
    const module = await import('~/widgets/documentForm')
    return module.DocumentForm
  })

  const sideContentComponent = computed(() => {
    if (isCreateDocument.value) {
      return DocumentForm
    }
    if (selectedDocument.value) {
      return DocumentFullItem
    }
    return null
  })

  const createHandle = () => {
    enableCreateDocument()
  }
</script>

<template>
  <BaseContainer>
    <template #main>
      <div class="main-content">
        <div class="main-content__header">
          <div class="main-content__title">
            Documents
            <BaseButton
              :disabled="isCreateDocument"
              size="medium"
              color="blue"
              @click="createHandle"
            >
              <template #icon>
                <PlusIcon />
              </template>
              Create new
            </BaseButton>
          </div>
        </div>
        <DocumentsList :documents="documents" />
      </div>
    </template>
    <template
      v-if="sideContentComponent"
      #additional
    >
      <div class="side-content">
        <component
          :is="sideContentComponent"
          :key="String(selectedDocument)"
        />
      </div>
    </template>
  </BaseContainer>
</template>

<style lang="scss" scoped>
  div {
    color: var(--text-primary);
  }

  .main-content {
    flex: 1;
  }

  .side-content {
    width: 100%;
    height: 100%;
    border-left: 1px solid var(--border-3);
    background: var(--surface-1);
  }

  .main-content__header {
    padding: 12px 24px;
    border-bottom: 1px solid var(--border-3);

    @include flex(row, flex-start, stretch);
  }

  .main-content__title {
    flex: 1;

    @include flex(row, space-between, center, 24px);
    @include font-inter-700(16px, 22px, var(--text-primary));
  }
</style>
