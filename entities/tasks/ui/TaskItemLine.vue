<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  // eslint-disable-next-line boundaries/element-types
  import { useTasksStore } from '~/features/task'
  import { AvatarsList } from '~/shared/ui/avatars'
  import type { Task } from '../model'
  import TaskStatusBadge from './TaskStatusBadge.vue'

  const props = defineProps<{
    task: Task
    isChild?: boolean
  }>()

  const tasksStore = useTasksStore()
  const selectedTask = computed(() => tasksStore.selectedTask)

  const isSelected = computed(() => {
    return (
      props.task.id === selectedTask.value?.id || selectedTask.value?.ancestry?.split('/').includes(`${props.task.id}`)
    )
  })

  const taskTitlePlaceholder = computed(() => {
    return props.task.title || `Task #${props.task.id}`
  })
</script>
<template>
  <div :class="['task-item', { selected: isSelected, 'is-child': isChild }]">
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
    background-color: var(--surface-2);
    outline: 2px solid transparent;
    transition: outline 0.08s ease-in-out;

    &.is-child {
      background-color: var(--surface-3);
    }

    &.selected {
      outline: 2px solid var(--button-primary);
    }

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
