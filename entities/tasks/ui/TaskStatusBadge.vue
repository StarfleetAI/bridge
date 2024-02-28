<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { useChangeCase } from '@vueuse/integrations/useChangeCase'
  import {
    TaskStatusTodo,
    TaskStatusInProgress,
    TaskStatusWaiting,
    TaskStatusPaused,
    TaskStatusDone,
    TaskStatusFailed,
    TaskStatusDraft,
  } from '~/shared/ui/icons'
  import { TaskStatus } from '../model'

  const props = defineProps<{
    status: TaskStatus
    complete?: number
    total?: number
    iconOnly?: boolean
  }>()
  const statusComponent = computed(() => {
    switch (props.status) {
      case TaskStatus.TODO:
        return TaskStatusTodo
      case TaskStatus.IN_PROGRESS:
        return TaskStatusInProgress
      case TaskStatus.WAITING_FOR_USER:
        return TaskStatusWaiting
      case TaskStatus.PAUSED:
        return TaskStatusPaused
      case TaskStatus.DONE:
        return TaskStatusDone
      case TaskStatus.FAILED:
        return TaskStatusFailed
      case TaskStatus.CANCELED:
        return TaskStatusFailed
      case TaskStatus.DRAFT:
      default:
        return TaskStatusDraft
    }
  })

  const statusToKebab = useChangeCase(props.status, 'paramCase')
  const showComplete = computed(() => {
    return typeof props.complete === 'number' && typeof props.total === 'number'
  })
</script>
<template>
  <div :class="['task-status', statusToKebab]">
    <component :is="statusComponent" />
    <span
      v-if="!iconOnly"
      class="task-status__label"
      >{{ props.status }}</span
    >
    <span
      v-if="showComplete"
      class="task-status__complete"
    >
      {{ props.complete }} / {{ props.total }}
    </span>
  </div>
</template>
<style scoped lang="scss">
  .task-status {
    display: flex;
    align-items: center;

    &.new,
    &.draft {
      color: var(--status-new);
    }

    &.todo {
      color: var(--status-todo);
    }

    &.in-progress {
      color: var(--status-progress);
    }

    &.waiting-for-user {
      color: var(--status-waiting);
    }

    &.paused {
      color: var(--status-paused);
    }

    &.done {
      color: var(--status-done);
    }

    &.failed {
      color: var(--status-failed);
    }

    &.canceled {
      color: var(--text-tertiary);
    }

    .icon {
      margin-right: 12px;
    }

    &__label {
      display: inline-block;
      margin-right: 8px;
      margin-left: 6px;
      color: inherit;

      @include font-inter-500(14px, 20px);
    }

    &__complete {
      display: inline-block;

      @include font-inter-400(14px, 20px, var(--text-tertiary));
    }
  }
</style>
