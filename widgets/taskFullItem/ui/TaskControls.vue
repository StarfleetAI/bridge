<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useTaskActions, useTasksNavigation } from '~/features/task'
  import type { Task } from '~/entities/tasks'
  import { BaseDropdown, BaseDropdownItem } from '~/shared/ui/dropdown'
  import { KebabIcon, CrossIcon } from '~/shared/ui/icons'

  const props = defineProps<{ task: Task }>()

  const { taskActions } = useTaskActions(toRef(() => props.task))

  const { setSelectedTask } = useTasksNavigation()
</script>

<template>
  <div class="task-controls">
    <BaseDropdown>
      <KebabIcon />
      <template #content>
        <BaseDropdownItem
          v-for="{ label, icon, action } in taskActions"
          :key="label"
          :class="['task-controls__action', { delete: label === 'Delete Task' }]"
          @click="action"
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

    @include flex($gap: 16px);
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
