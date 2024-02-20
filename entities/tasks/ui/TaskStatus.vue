<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import {
    TaskStatusTodo,
    TaskStatusInProgress,
    TaskStatusWaiting,
    TaskStatusPaused,
    TaskStatusDone,
    TaskStatusFailed,
    TaskStatusNew,
  } from '~/shared/ui/icons'
  import { type Status } from '../model'

  const props = defineProps<{
    status: Status
    complete: number
    total: number
  }>()
  const statusComponent = computed(() => {
    switch (props.status) {
      case 'Todo':
        return TaskStatusTodo
      case 'InProgress':
        return TaskStatusInProgress
      case 'WaitingForUser':
        return TaskStatusWaiting
      case 'Paused':
        return TaskStatusPaused
      case 'Done':
        return TaskStatusDone
      case 'Failed':
        return TaskStatusFailed
      case 'Canceled':
        return TaskStatusFailed
      case 'New':
        return TaskStatusNew
      default:
        return null
    }
  })

  const statusColor = computed(() => {
    switch (props.status) {
      case 'Todo':
        return '#32AAED'
      case 'InProgress':
        return '#62AC55'
      case 'WaitingForUser':
        return '#DA893E'
      case 'Paused':
        return '#9E83EA'
      case 'Done':
        return '#00A140'
      case 'Failed':
        return '#DC493E'
      case 'Canceled':
        return '#DC493E'
      case 'New':
        return '#677383'
      default:
        return 'transparent'
    }
  })
</script>
<template>
  <div
    class="task-status"
    :style="{ color: statusColor }"
  >
    <component :is="statusComponent" />
    <span
      class="task-status__label"
      :style="{ color: statusColor }"
      >{{ props.status }}</span
    >
    <span class="task-status__complete"> {{ props.complete }} / {{ props.total }} </span>
  </div>
</template>
<style scoped lang="scss">
    .task-status {
      display: flex;
      align-items: center;

      .icon {
        margin-right: 12px;
      }

      &__label {
        display: inline-block;
        margin-right: 8px;
  <<<<<<< HEAD
        margin-left: 8px;
  =======
        margin-left: 6px;
  >>>>>>> main

        @include font-inter-500(14px, 20px);
      }

      &__complete {
        display: inline-block;

        @include font-inter-400(14px, 20px, var(--text-tertiary));
      }
    }
</style>
