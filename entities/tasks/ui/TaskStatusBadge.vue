<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { useChangeCase } from '@vueuse/integrations/useChangeCase'
  import {
    TaskStatusToDo,
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
    let Component = TaskStatusDraft
    switch (props.status) {
      case TaskStatus.TODO:
        Component = TaskStatusToDo
        break
      case TaskStatus.IN_PROGRESS:
        Component = TaskStatusInProgress
        break
      case TaskStatus.WAITING_FOR_USER:
        Component = TaskStatusWaiting
        break
      case TaskStatus.PAUSED:
        Component = TaskStatusPaused
        break
      case TaskStatus.DONE:
        Component = TaskStatusDone
        break
      case TaskStatus.FAILED:
        Component = TaskStatusFailed
        break
      case TaskStatus.DRAFT:
      default:
        Component = TaskStatusDraft
    }
    return defineAsyncComponent(Component)
  })

  const statusToKebab = computed(() => useChangeCase(props.status, 'paramCase').value)
  const statusToNormal = computed(() => useChangeCase(props.status, 'sentenceCase').value)
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
    >
      {{ statusToNormal }}
    </span>
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
