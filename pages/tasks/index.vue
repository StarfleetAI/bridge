<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { TaskForm } from '~/widgets/taskForm'
  import { TaskFullItem } from '~/widgets/taskFullItem'
  import { TasksList } from '~/widgets/tasksList'
  import { useTasksStore } from '~/features/task'
  import { BaseContainer } from '~/shared/ui/base'

  definePageMeta({
    title: 'Tasks',
  })

  const { isNewTask, isNewTaskQuery, selectedTaskQuery } = storeToRefs(useTasksStore())

  const sideContentComponent = computed(() => {
    if (isNewTask.value || isNewTaskQuery.value) {
      return TaskForm
    }
    if (selectedTaskQuery.value) {
      return TaskFullItem
    }
    return null
  })
</script>

<template>
  <BaseContainer>
    <template #main>
      <TasksList />
    </template>
    <template
      v-if="sideContentComponent"
      #additional
    >
      <div
        v-if="sideContentComponent"
        class="side-content"
      >
        <component :is="sideContentComponent" />
      </div>
    </template>
  </BaseContainer>
</template>

<style lang="scss" scoped>
  div {
    color: var(--text-primary);
  }

  .side-content {
    width: 100%;
    height: 100%;
    background: var(--surface-2);
  }
</style>
