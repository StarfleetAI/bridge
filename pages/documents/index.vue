<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  // import { useTasksStore } from '@/store/tasks'
  import { DocumentFullItem } from '~/widgets/DocumentFullItem'
  import { DocumentsList } from '~/widgets/documentsList'
  import { DocumentTitleIcon } from '~/shared/icons'

  definePageMeta({
    title: 'Tasks'
  })

  // const tasksStore = useTasksStore()
</script>

<template>
  <div class="main-content">
    <div class="documents-list__title">
      <DocumentTitleIcon :color="'var(--text-primary)'" />
      Documents <span>4</span>
    </div>
    <DocumentsList />
  </div>
  <div class="side-content">
    <DocumentFullItem />
  </div>
</template>

<style lang="scss" scoped>
  div {
    color: var(--text-primary);
  }

  .main-content {
    width: 60%;
    min-height: calc(100vh - 44px);
    padding: 20px;
  }

  .side-content {
    overflow-y: auto;
    width: 40%;
    min-height: calc(100vh - 44px);
    background: var(--side-panel);
  }

  .documents-list {
    &__title {
      gap: 8px;
      margin-bottom: 24px;

      span {
        @include font-inter-400(16px, 22px, var(--text-tertiary));
      }

      @include flex(row, start, center);
      @include font-inter-700(16px, 22px, var(--text-secondary));
    }
  }
</style>
