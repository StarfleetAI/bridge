<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAgentsStore } from '~/features/agent'
  import { useTasksStore } from '~/features/task'
  import type { TasksGroupName } from '~/features/task'
  import { TaskItemLine, TaskStatus, type Task } from '~/entities/tasks'
  import { BaseButton } from '~/shared/ui/base'
  import { ArrowLeftIcon, PlusIcon } from '~/shared/ui/icons'
  import { BasePagination as BasePaginationAsync } from '~/shared/ui/pagination'
  import type { TasksListView } from '../model'

  const BasePagination = defineAsyncComponent(BasePaginationAsync)

  const view = ref<TasksListView>('list')

  const { listRootTasksByStatus, selectTask, setIsNewTask, setGroup, setPage } = useTasksStore()
  const { tasksGroupsByStatus, isNewTask, selectedTask, selectedGroup, currentPage, totalPages } =
    storeToRefs(useTasksStore())

  await listRootTasksByStatus()
  // const toggleView = (value: TasksListView) => {
  //   view.value = value
  // }

  const getGroupName = (status: TaskStatus): TasksGroupName => {
    switch (status) {
      case TaskStatus.DRAFT:
        return 'Drafts'
      case TaskStatus.TODO:
        return 'To Do'
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
  const isSelected = (task: Task) => {
    return (
      task.id === selectedTask.value?.id || selectedTask.value?.ancestry?.split('/').includes(`${task.id}`) || false
    )
  }
  const agentsStore = useAgentsStore()

  const tasksListRef = ref<HTMLDivElement>()
  const navigateToGroup = async (group: Nullable<TaskStatus>) => {
    await setGroup(group)
    tasksListRef.value?.scroll({ top: 0 })
  }

  const handlePageUpdate = async (page: number) => {
    setPage(page)
    await listRootTasksByStatus()
    await nextTick()
    tasksListRef.value?.scroll({ top: 0 })
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
    <div
      ref="tasksListRef"
      class="tasks-list__groups"
    >
      <div
        v-for="(group, status) in tasksGroupsByStatus"
        :key="status"
        class="tasks-list__group"
      >
        <template v-if="group.tasks.length">
          <div
            class="tasks-list__group-name"
            @click="navigateToGroup(status)"
          >
            <ArrowLeftIcon
              v-if="selectedGroup"
              color="var(--text-secondary)"
              @click.stop="navigateToGroup(null)"
            />
            <b>{{ getGroupName(status) }}</b>
            {{ group.count }}
          </div>
          <div :class="{ 'tasks-list__group--list': view === 'list', 'tasks-list__group--grid': view === 'grid' }">
            <TaskItemLine
              v-for="task in group.tasks"
              :key="task.id"
              :task="task"
              :is-selected="isSelected(task)"
              :task-agent="agentsStore.getById(task.agent_id)"
              @click="selectTask(task.id)"
            />
          </div>
          <BasePagination
            v-if="selectedGroup && totalPages > 1"
            v-model="currentPage"
            :max-page-visible="5"
            :total-pages="totalPages"
            @update:model-value="handlePageUpdate"
          />
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

    b {
      color: var(--text-secondary);
    }

    @include font-inter-400(14px, 20px, var(--text-tertiary));
    @include flex($align: center, $gap: 8px);
  }
</style>
