<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { useRouteQuery } from '@vueuse/router'
  import { AvatarsList } from '~/shared/ui/avatars'
  import type { Task } from '../model'
  import TaskStatusBadge from './TaskStatusBadge.vue'

  const props = defineProps<{
    task: Task
  }>()

  const selectedTask = useRouteQuery('task', '', {
    transform: (value: string) => (isNaN(Number(value)) ? null : Number(value)),
  })

  const isSelected = computed(() => {
    return props.task.id === selectedTask.value
  })

  const taskTitlePlaceholder = computed(() => {
    return props.task.title || `Task #${props.task.id}`
  })
</script>
<template>
  <div :class="['task-item', { selected: isSelected }]">
    <TaskStatusBadge
      icon-only
      :status="task.status"
    />
    <div class="task-item__content">
      {{ taskTitlePlaceholder }}
    </div>
    <AvatarsList
      class="task-item__avatars"
      :persons="[
        { name: 'Alex', avatar: '', link: '' },
        { name: 'Robert', avatar: '', link: '' },
      ]"
    />
  </div>
</template>
<style scoped lang="scss">
  .task-item {
    flex: 1;
    margin-bottom: 15px;
    padding: 12px 16px;
    border-radius: 6px;
    background: var(--surface-2);

    @include font-inter-400(14px, 20px, var(--text-primary));
    @include flex(row, flex-start, center, 8px);
  }

  .task-item__content {
    @include line-clamp(1);
  }

  .task-item__avatars {
    margin-left: auto;
  }
</style>
