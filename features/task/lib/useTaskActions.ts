import { type Task, TaskStatus } from '~/entities/tasks'
import { CancelIcon, DeleteIcon, DuplicateIcon, PauseIcon, ReviseIcon } from '~/shared/ui/icons'
import { duplicateTask, pauseTask, reviseTask, cancelTask } from '../api'
import { useTasksStore } from '../store'
import { useTasksNavigation } from './useTasksNavigation'

export const useTaskActions = (task: Ref<Task>) => {
  const id = computed(() => task.value.id)
  const status = computed(() => task.value.status)

  const duplicate = computed(() => {
    return {
      label: 'Duplicate',
      icon: DuplicateIcon,
      action: () => duplicateTask(id.value),
    }
  })

  const { setSelectedTask } = useTasksNavigation()
  const { deleteTask: deleteTaskReq } = useTasksStore()

  const deleteTask = computed(() => {
    return {
      label: 'Delete Task',
      icon: DeleteIcon,
      action: async () => {
        await deleteTaskReq(id.value)
        setSelectedTask(null)
      },
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
