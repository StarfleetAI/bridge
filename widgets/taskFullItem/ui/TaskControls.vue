<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useTaskActions, useTasksNavigation, useTasksStore } from '~/features/task'
  import { type Task, TaskStatus } from '~/entities/tasks'
  import { BaseButton } from '~/shared/ui/base'
  import { BaseDropdown, BaseDropdownItem } from '~/shared/ui/dropdown'
  import { KebabIcon, CrossIcon, StarsIcon } from '~/shared/ui/icons'

  const props = defineProps<{ task: Task }>()
  const { taskActions } = useTaskActions(toRef(() => props.task))
  const { executeTask } = useTasksStore()
  const { setSelectedTask } = useTasksNavigation()

  const handleActionClick = (action: () => Promise<Task | void>, isDelete = false, isDuplicate = false) => {
    action().then((updatedTask) => {
      if (isDelete) {
        navigateTo('/tasks')
      } else if (isDuplicate) {
        setSelectedTask(updatedTask!.id)
      }
    })
  }

  const handleExecuteTask = async () => {
    await executeTask(props.task.id)
  }
</script>

<template>
  <div class="task-controls">
    <BaseButton
      v-if="task.status === TaskStatus.DRAFT"
      class="task-details__execute"
      :disabled="task.summary.length === 0"
      @click="handleExecuteTask"
    >
      <template #icon>
        <StarsIcon />
      </template>
      Execute
    </BaseButton>
    <BaseDropdown>
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
          @click="handleActionClick(action, label === 'Delete Task', label === 'Duplicate')"
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
      @click="setSelectedTask(null)"
    />
  </div>
</template>

<style lang="scss" scoped>
  .task-controls {
    color: var(--text-tertiary);

    @include flex($gap: 16px, $align-items: center);
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
