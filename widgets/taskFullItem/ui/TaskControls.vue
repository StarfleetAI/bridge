<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useTaskActions, useTasksStore } from '~/features/task'
  import { type Task, TaskStatus } from '~/entities/tasks'
  import { BaseButton } from '~/shared/ui/base'
  import { BaseDropdown, BaseDropdownItem } from '~/shared/ui/dropdown'
  import { KebabIcon, CrossIcon, StarsIcon, ReviseIcon /**ResumeIcon*/ } from '~/shared/ui/icons'
  const StarsIconAsync = defineAsyncComponent(StarsIcon)
  // const ResumeIconAsync = defineAsyncComponent(ResumeIcon)
  const props = defineProps<{ task: Task }>()
  const { taskActions } = useTaskActions(toRef(() => props.task))
  const { executeTask, selectTask, reviseTask } = useTasksStore()

  const showReviseButton = computed(() => {
    return [TaskStatus.TODO, TaskStatus.IN_PROGRESS, TaskStatus.FAILED, TaskStatus.WAITING_FOR_USER].includes(
      props.task.status,
    )
  })
  const showReExecuteButton = computed(() => {
    return [TaskStatus.FAILED, TaskStatus.DONE].includes(props.task.status)
  })
</script>
<template>
  <div class="task-controls">
    <BaseButton
      v-if="showReviseButton"
      color="gray"
      size="medium"
      @click="reviseTask(task.id)"
    >
      <template #icon>
        <ReviseIcon />
      </template>
      Revise
    </BaseButton>
    <!-- <BaseButton v-if="task.status === TaskStatus.WAITING_FOR_USER">
      <template #icon>
        <ResumeIconAsync />
        Continue
      </template>
    </BaseButton> -->
    <BaseButton
      v-if="task.status === TaskStatus.DRAFT"
      class="task-details__execute"
      :disabled="task.title.length === 0"
      @click="executeTask(task.id)"
    >
      <template #icon>
        <StarsIconAsync />
      </template>
      Execute
    </BaseButton>
    <BaseButton
      v-if="showReExecuteButton"
      class="task-details__execute"
      :disabled="task.title.length === 0"
      @click="executeTask(task.id)"
    >
      <template #icon>
        <StarsIconAsync />
      </template>
      Re-execute
    </BaseButton>
    <BaseDropdown :distance="20">
      <KebabIcon
        height="20"
        width="20"
      />
      <template #content>
        <BaseDropdownItem
          v-for="{ label, icon, action } in taskActions"
          :key="label"
          v-close-popper
          :class="['task-controls__action', { delete: label === 'Delete Task' }]"
          @click="action()"
        >
          <template #icon>
            <component
              :is="icon"
              class="task-controls__action-icon"
            />
          </template>
          <template #label>
            <div class="task-controls__action-label">
              {{ label }}
            </div>
          </template>
        </BaseDropdownItem>
      </template>
    </BaseDropdown>
    <CrossIcon
      width="20"
      height="20"
      @click="selectTask(null)"
    />
  </div>
</template>

<style lang="scss" scoped>
  .task-controls {
    color: var(--text-tertiary);

    @include flex($gap: 16px, $align: center);
  }

  .task-controls__action {
    &.delete {
      color: var(--status-failed);
    }

    &:hover {
      background-color: var(--surface-4);

      &:not(.delete) {
        color: var(--text-primary);
      }
    }

    @include font-inter-500(16px, 22px, var(--text-secondary));
    @include flex(row, flex-start, center, 8px);
  }
</style>
