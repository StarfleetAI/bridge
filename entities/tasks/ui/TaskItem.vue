<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { AvatarsList } from '~/shared/ui/avatars'
  import { InlineFiles } from '~/shared/ui/files'
  import type { Task } from '../model'
  import TaskStatusBadge from './TaskStatusBadge.vue'

  const props = defineProps<{
    task: Task
  }>()
</script>
<template>
  <div class="task-list-item">
    <div class="task-list-item__head">
      <TaskStatusBadge
        :status="task.status"
        :complete="1"
        :total="2"
      />
      <AvatarsList
        :persons="[
          { name: 'Alex', avatar: '', link: '' },
          { name: 'Robert', avatar: '', link: '' },
        ]"
      />
    </div>
    <div class="task-list-item__body">
      {{ task.summary }}
    </div>
    <div class="task-list-item__footer">
      <InlineFiles
        :files="[
          { type: 'TXT', url: 'file.txt', name: 'file.txt' },
          { type: 'TXT', url: 'file.txt', name: 'file.txt' },
          { type: 'TXT', url: 'file.txt', name: 'file.txt' },
        ]"
      />
    </div>
  </div>
</template>
<style scoped lang="scss">
  .task-list-item {
    margin-bottom: 15px;
    padding: 12px 16px;
    border-radius: 6px;
    background: var(--side-panel);

    &__head {
      @include flex(row, space-between, space-between);
    }

    &__body {
      padding: 12px 0;

      @include font-inter-500(16px, 22px, var(--text-secondary));
      @include flex(row, space-between, space-between);
    }

    &__footer {
      @include flex(row, space-between, space-between);
    }
  }
</style>
