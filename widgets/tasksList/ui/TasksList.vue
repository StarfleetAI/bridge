<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useTasksNavigation, useTasksStore } from '~/features/task'
  import { TaskItem } from '~/entities/tasks'
  import { type Pagination } from '~/shared/model'
  import type { TasksListView } from '../model'
  import CreateTaskButton from './CreateTaskButton.vue'

  const view = ref<TasksListView>('list')
  const { listRootTasks } = useTasksStore()
  const pagination = ref<Pagination>({
    page: 1,
    per_page: 14,
  })
  await listRootTasks({ pagination: pagination.value })
  const { tasksGroupsByStatus } = storeToRefs(useTasksStore())
  const toggleView = (value: TasksListView) => {
    view.value = value
  }
  const { enableCreateTask, isCreateTask } = useTasksNavigation()
</script>

<template>
  <div class="tasks-list__header">
    <div class="tasks-list__title">
      Tasks
      <CreateTaskButton
        :disabled="isCreateTask"
        @click="enableCreateTask"
      />
    </div>
    <div class="tasks-list__views" />
  </div>
  <div
    class="tasks-list"
    :class="{ 'tasks-list--list': view === 'list', 'tasks-list--grid': view === 'grid' }"
  >
    <div
      v-for="[status, tasks] in tasksGroupsByStatus"
      :key="status"
      class="tasks-list__group"
    >
      <div class="tasks-list__group-name">
        <b>{{ status }}</b> {{ tasks.length }}
      </div>
      <TaskItem
        v-for="task in tasks"
        :key="task.id"
        :task="task"
      />
    </div>
  </div>
</template>
<style scoped lang="scss">
  .tasks-list {
    display: grid;
    columns: 2;
  }

  .tasks-list__header {
    padding: 12px 24px;
    border-bottom: 1px solid var(--border-3);

    @include flex(row, space-between, stretch);
  }

  .tasks-list__title {
    flex: 1;

    @include flex(row, space-between, center);
    @include font-inter-700(16px, 22px, var(--text-primary));
  }

  .tasks-list__group {
    @include flex(column);
  }

  .tasks-list__group-name {
    padding: 16px 24px;

    @include font-inter-400(14px, 20px, var(--text-tertiary));
  }
</style>
