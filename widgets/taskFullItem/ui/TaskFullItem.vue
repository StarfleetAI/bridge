<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { TaskStatus } from '~/entities/tasks'
  import { AvatarsList } from '~/shared/ui/avatars'
  import { FilesList, LargeFilesPreview } from '~/shared/ui/files'
  import { TaskHeaderIcon, DocumentIcon, ResultIcon, ArrowLeftIcon } from '~/shared/ui/icons'
</script>
<template>
  <div class="task-full-item">
    <div class="task-full-item__head">
      <div class="task-full-item__title">
        <TaskHeaderIcon />
        Task #0393 • 14.07.2024, 18:32
      </div>
    </div>
    <div class="task-full-item__body">
      <div class="task-full-item__back">
        <ArrowLeftIcon /> Define the key requirements from the client for Brand Analytics functionality.
      </div>
      <div class="task-full-item__top">
        <div class="task-full-item__status">
          <TaskStatus
            status="Todo"
            :complete="1"
            :total="2"
          />
        </div>
        <div class="task-full-item__status">
          <AvatarsList :persons="[{ name: 'Product designer', avatar: '', link: '' }]" />
        </div>
      </div>
      <div class="task-full-item__middle">
        <div class="task-full-item__text-title">
          Develop a project schedule considering the stages of neural network development and service implementation.
        </div>
        <div class="task-full-item__text">
          This task involves identifying and implementing robust methods to guarantee the security of data at every
          stage of Brand Analytics, from collection through storage to analysis. It is crucial to establish a
          comprehensive approach that safeguards sensitive information and maintains the integrity and confidentiality
          of the data processed by the neural network. Below is a list of key actions and considerations for ensuring
          data security
        </div>
        <div class="task-full-item__attachments">
          <div><DocumentIcon /> Linked Documents</div>
          <div>+ Add</div>
        </div>
        <FilesList
          :files="[
            {
              type: 'TXT',
              url: 'file.txt',
              name: 'file.txt',
              created: '14.07.2024, 18:32',
              rows: 10,
              size: '1.2 MB'
            },
            {
              type: 'TXT',
              url: 'file.txt',
              name: 'file.txt',
              created: '14.07.2024, 18:32',
              rows: 10,
              size: '1.2 MB'
            },
            {
              type: 'TXT',
              url: 'file.txt',
              name: 'file.txt',
              created: '14.07.2024, 18:32',
              rows: 10,
              size: '1.2 MB'
            }
          ]"
        />
      </div>
    </div>
    <div class="task-full-item__result">
      <div class="task-full-item__result-head"><ResultIcon /> Result</div>
      <LargeFilesPreview
        :files="[
          {
            type: 'TXT',
            url: 'file.txt',
            name: 'file.txt',
            created: '14.07.2024, 18:32',
            rows: 10,
            size: '1.2 MB'
          },
          {
            type: 'TXT',
            url: 'file.txt',
            name: 'file.txt',
            created: '14.07.2024, 18:32',
            rows: 10,
            size: '1.2 MB'
          },
          {
            type: 'TXT',
            url: 'file.txt',
            name: 'file.txt',
            created: '14.07.2024, 18:32',
            rows: 10,
            size: '1.2 MB'
          }
        ]"
      />
      <div class="task-full-item__result-text-wrapper">
        <div class="task-full-item__result-title">Stage 1</div>
        <div class="task-full-item__result-text">
          This task involves identifying and implementing robust methods to guarantee the security of data at every
          stage of Brand Analytics, from collection through storage to analysis. It is crucial to establish a
          comprehensive approach that safeguards sensitive information and maintains the integrity and confidentiality
          of the data processed by the neural network.
        </div>
      </div>
    </div>
  </div>
</template>
<style scoped lang="scss">
  .task-full-item {
    &__head {
      padding: 12px 24px;
      border-bottom: 0.5px solid var(--pill);

      @include flex(row, space-between, space-between);
    }

    &__title {
      gap: 4px;

      @include font-inter-500(14px, 22px, var(--text-tertiary));
      @include flex(row, space-between, space-between);
    }

    &__back {
      gap: 4px;
      padding: 8px;
      background: var(--surface-3);
      cursor: pointer;

      @include flex(row, flex-start, space-between);
      @include font-inter-700(12px, 22px, var(--text-tertiary));
    }

    &__body {
      padding: 24px;
      border-bottom: 0.5px solid var(--pill);
    }

    &__top {
      padding: 24px 0;

      @include flex(row, space-between, space-between);
    }

    &__text-title {
      margin-bottom: 16px;

      @include font-inter-500(16px, 22px, var(--text-primary));
    }

    &__text {
      position: relative;

      &:after {
        content: '';
        position: absolute;
        right: 0;
        bottom: 0;
        left: 0;
        height: 50px;
        background: linear-gradient(to bottom, rgba(36, 39, 49, 0), rgba(36, 39, 49, 1));
      }

      @include font-inter-400(14px, 19px, var(--text-secondary));
    }

    &__attachments {
      padding: 8px 0;

      div {
        @include flex(row, space-between, center);
      }

      @include font-inter-500(14px, 22px, var(--text-tertiary));
      @include flex(row, space-between, center);
    }

    &__result {
      padding: 24px;
    }

    &__result-head {
      gap: 2px;
      margin-bottom: 24px;

      @include font-inter-500(14px, 22px, var(--text-tertiary));
      @include flex(row, flex-start, center);
    }

    &__result-text-wrapper {
      padding: 12px;
      border-radius: 12px;
      background: var(--surface-3);
    }

    &__result-title {
      margin-bottom: 24px;

      @include font-inter-500(16px, 25px, var(--text-secondary));
    }

    &__result-text {
      @include font-inter-500(14px, 22px, var(--text-secondary));
    }
  }
</style>
