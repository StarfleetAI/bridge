// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Task, TaskStatus } from '~/entities/tasks'
import { DeleteIcon, DuplicateIcon, ReviseIcon } from '~/shared/ui/icons'
import { useTasksStore } from '../store'

export const useTaskActions = (task: Ref<Task>) => {
  const id = computed(() => task.value.id)
  const status = computed(() => task.value.status)
  const { deleteTask: deleteTaskReq, duplicateTask, planTask, selectTask } = useTasksStore()

  const duplicate = computed(() => {
    return {
      label: 'Duplicate',
      icon: DuplicateIcon,
      action: async () => {
        const newTask = await duplicateTask(task.value.id)
        selectTask(newTask.id)
      },
    }
  })

  const deleteTask = computed(() => {
    return {
      label: 'Delete Task',
      icon: DeleteIcon,
      action: () => {
        selectTask(null)
        deleteTaskReq(task.value)
      },
    }
  })

  const plan = computed(() => {
    return {
      label: 'Plan',
      icon: ReviseIcon,
      action: () => planTask(id.value),
    }
  })
  const defaultActions = computed(() => {
    return [duplicate.value, deleteTask.value]
  })
  const extendedActions = computed(() => {
    return [plan.value, duplicate.value, deleteTask.value]
  })

  const taskActions = computed(() => {
    switch (status.value) {
      case TaskStatus.TODO:
      case TaskStatus.IN_PROGRESS:
        return defaultActions.value
      default:
        return extendedActions.value
    }
  })

  return { taskActions }
}
