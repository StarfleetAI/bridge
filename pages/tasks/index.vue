<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  // import { useTasksStore } from '@/store/tasks'

  import { TaskFullItem } from '~/widgets/taskFullItem'
  import { TasksList } from '~/widgets/tasksList'

  definePageMeta({
    title: 'Tasks',
  })

  // const tasksStore = useTasksStore()
</script>

<template>
  <div class="main-content">
    <TasksList />
  </div>
  <div class="side-content">
    <TaskFullItem />
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
</style>
