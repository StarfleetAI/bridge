// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Task, TaskStatus } from '~/entities/tasks'
import { CancelIcon, DeleteIcon, DuplicateIcon, PauseIcon, ReviseIcon } from '~/shared/ui/icons'
import { useTasksStore } from '../store'

export const useTaskActions = (task: Ref<Task>) => {
  const id = computed(() => task.value.id)
  const status = computed(() => task.value.status)
  const { deleteTask: deleteTaskReq, reviseTask, cancelTask, pauseTask, duplicateTask } = useTasksStore()

  const duplicate = computed(() => {
    return {
      label: 'Duplicate',
      icon: DuplicateIcon,
      action: () => duplicateTask(task.value.id),
    }
  })

  const deleteTask = computed(() => {
    return {
      label: 'Delete Task',
      icon: DeleteIcon,
      action: () => deleteTaskReq(id.value),
    }
  })

  const revise = computed(() => {
    return {
      label: 'Revise',
      icon: ReviseIcon,
      action: () => reviseTask(id.value),
    }
  })

  const cancel = computed(() => {
    return {
      label: 'Cancel',
      icon: CancelIcon,
      action: () => cancelTask(id.value),
    }
  })

  const pause = computed(() => {
    return {
      label: 'Pause',
      icon: PauseIcon,
      action: () => pauseTask(id.value),
    }
  })

  const baseActions = computed(() => {
    return [duplicate.value, deleteTask.value]
  })

  const todoActions = computed(() => {
    return [revise.value, deleteTask.value]
  })

  const inProgressActions = computed(() => {
    return [pause.value, cancel.value, duplicate.value, deleteTask.value]
  })

  const pausedActions = computed(() => {
    return [revise.value, cancel.value, duplicate.value, deleteTask.value]
  })

  const waitingActions = computed(() => {
    return [pause.value, revise.value, duplicate.value, deleteTask.value]
  })

  const taskActions = computed(() => {
    switch (status.value) {
      case TaskStatus.TODO:
        return todoActions.value
      case TaskStatus.IN_PROGRESS:
        return inProgressActions.value
      case TaskStatus.PAUSED:
        return pausedActions.value
      case TaskStatus.WAITING_FOR_USER:
        return waitingActions.value
      default:
        return baseActions.value
    }
  })

  return { taskActions }
}
