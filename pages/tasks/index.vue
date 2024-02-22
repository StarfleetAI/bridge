<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { TasksList } from '~/widgets/tasksList'
  import { useTasksNavigation } from '~/features/task'
  import { BaseContainer } from '~/shared/ui/base'

  definePageMeta({
    title: 'Tasks',
  })

  const { isCreateTask, selectedTask } = useTasksNavigation()
  const TaskFullItem = defineAsyncComponent(async () => {
    const module = await import('~/widgets/taskFullItem')
    return module.TaskFullItem
  })
  const TaskForm = defineAsyncComponent(async () => {
    const module = await import('~/widgets/taskForm')
    return module.TaskForm
  })
  const sideContentComponent = computed(() => {
    if (isCreateTask.value) {
      return TaskForm
    }
    if (selectedTask.value) {
      return TaskFullItem
    }
    return null
  })
</script>

<template>
  <BaseContainer>
    <template #main>
      <div class="main-content">
        <TasksList />
      </div>
    </template>
    <template #additional>
      <div class="side-content">
        <component :is="sideContentComponent" />
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
    background: var(--surface-2);
  }
</style>
