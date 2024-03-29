<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useTasksStore } from '~/features/task'
  import type { TasksGroupName } from '~/features/task'
  import { TaskItemLine, TaskStatus } from '~/entities/tasks'
  import { BaseButton } from '~/shared/ui/base'
  import { PlusIcon } from '~/shared/ui/icons'
  import type { TasksListView } from '../model'

  const view = ref<TasksListView>('list')
  const { listRootTasksByStatus, selectTask, setIsNewTask } = useTasksStore()
  await listRootTasksByStatus()
  const { tasksGroupsByStatus, isNewTask } = storeToRefs(useTasksStore())
  // const toggleView = (value: TasksListView) => {
  //   view.value = value
  // }

  const getGroupName = (status: TaskStatus): TasksGroupName => {
    switch (status) {
      case TaskStatus.DRAFT:
        return 'Drafts'
      case TaskStatus.TODO:
        return 'To Do'
      case TaskStatus.PAUSED:
        return 'Paused'
      case TaskStatus.WAITING_FOR_USER:
        return 'Waiting For User'
      case TaskStatus.IN_PROGRESS:
        return 'In Progress'
      case TaskStatus.DONE:
        return 'Done'
      case TaskStatus.FAILED:
        return 'Failed'
    }
  }
</script>

<template>
  <div class="tasks-list">
    <div class="tasks-list__header">
      <div class="tasks-list__title">
        Tasks
        <BaseButton
          :disabled="isNewTask"
          size="medium"
          color="blue"
          type="solid"
          class="task-list__create"
          @click="setIsNewTask(true)"
        >
          <template #icon>
            <PlusIcon />
          </template>
          Create new
        </BaseButton>
      </div>
      <div class="tasks-list__views" />
    </div>
    <div class="tasks-list__groups">
      <div
        v-for="(tasks, status) in tasksGroupsByStatus"
        :key="status"
        class="tasks-list__group"
      >
        <template v-if="tasks.length">
          <div class="tasks-list__group-name">
            <b>{{ getGroupName(status) }}</b> {{ tasks.length }}
          </div>
          <div :class="{ 'tasks-list__group--list': view === 'list', 'tasks-list__group--grid': view === 'grid' }">
            <TaskItemLine
              v-for="task in tasks"
              :key="task.id"
              :task="task"
              @click="selectTask(task.id)"
            />
          </div>
        </template>
      </div>
    </div>
  </div>
</template>
<style scoped lang="scss">
  .tasks-list {
    flex: 1;
    width: 100%;

    @include flex(column, flex-start, stretch);
  }

  .tasks-list__header {
    height: 56px;
    padding: 12px 24px;
    border-bottom: 0.5px solid var(--border-3);

    @include flex(row, flex-start, stretch);
  }

  .tasks-list__title {
    flex: 1;

    @include flex(row, flex-start, center, 24px);
    @include font-inter-700(16px, 22px, var(--text-primary));
  }

  .tasks-list__groups {
    overflow: hidden auto;
    padding: 12px 24px;

    @include flex(column);
    @include add-scrollbar;
  }

  .tasks-list__group {
    &--grid {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 12px;
    }

    &--list {
      @include flex(column);
    }

    @include flex(column);
  }

  .tasks-list__group-name {
    padding: 16px 0;

    @include font-inter-400(14px, 20px, var(--text-tertiary));
  }
</style>
