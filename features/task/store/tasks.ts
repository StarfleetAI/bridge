// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { listen } from '@tauri-apps/api/event'
import { useRouteQuery } from '@vueuse/router'
// eslint-disable-next-line boundaries/element-types
import { useChatsStore } from '~/features/chats'
import { TaskStatus, type Task, type SelectedTask } from '~/entities/tasks'
import { usePagination, useToast } from '~/shared/lib'
import {
  listRootTasks as listRootTasksReq,
  listChildTasks as listChildTasksReq,
  listRootTasksByStatus as listRootTasksByStatusReq,
  getTask,
  createTask as createTaskReq,
  deleteTask as deleteTaskReq,
  updateTask as updateTaskReq,
  reviseTask as reviseTaskReq,
  executeTask as executeTaskReq,
  duplicateTask as duplicateTaskReq,
  planTask as planTaskReq,
} from '../api'
import { type CreateTask, type GroupedTasks, type ListTasksParams, type UpdateTask } from '../model'

export const useTasksStore = defineStore('tasks', () => {
  const { errorToast } = useToast()
  const tasks = ref<Task[]>([])
  const selectedTask = ref<Nullable<SelectedTask>>(null)
  const selectedTaskQuery = useRouteQuery<Nullable<number>>('task', () => null, {
    transform(val) {
      if (val) {
        return Number(val)
      }
      return null
    },
  })
  const selectedGroup = useRouteQuery<Nullable<TaskStatus>>('group', () => null)
  const setGroup = async (val: Nullable<TaskStatus> = null) => {
    if (selectedGroup.value === val) {
      return
    }
    setPage(1)
    isNewTask.value = false
    isNewTaskQuery.value = false
    selectedTask.value = null
    selectedTaskQuery.value = null
    selectedGroup.value = val

    listRootTasksByStatus()
  }

  const getLastAncestor = (ancestry: string | undefined) => {
    if (ancestry) {
      const parts = ancestry.split('/')
      return isNaN(Number(parts.at(-1))) ? null : Number(parts.at(-1))
    }
    return null
  }
  const selectedTaskParentId = computed(() => {
    const lastAncestor = getLastAncestor(selectedTask.value?.ancestry)
    if (lastAncestor) {
      return isNaN(Number(lastAncestor)) ? null : Number(lastAncestor)
    }
    return null
  })

  const isNewTask = ref(false)
  const isNewTaskQuery = useRouteQuery('create', 'false', { transform: (value: 'false' | 'true') => value === 'true' })
  const route = useRoute()

  const setIsNewTask = (val: boolean) => {
    selectedTask.value = null
    selectedTaskQuery.value = null
    isNewTask.value = val
    isNewTaskQuery.value = val
    if (val) {
      navigateTo({ path: '/tasks', query: { ...route.query, create: 'true', task: null } })
    } else {
      navigateTo({ path: '/tasks', query: { ...route.query, create: 'false', task: null } })
    }
  }
  const selectTask = async (id: Nullable<number>) => {
    try {
      isNewTask.value = false
      isNewTaskQuery.value = false
      if (id) {
        navigateTo({ path: '/tasks', query: { ...route.query, task: id, create: 'false' } })
        const [task, children] = await Promise.all([getTask(id), listChildTasksReq(id)])
        selectedTask.value = { ...task, children }
        selectedTaskQuery.value = task.id
      } else {
        await navigateTo({ path: '/tasks', query: { ...route.query, create: 'false', task: null } })
        selectedTask.value = null
        selectedTaskQuery.value = null
      }
    } catch (error) {
      errorToast(String(error))
    }
  }
  const getDefaultTasksGroupsByStatus = () => {
    return {
      Draft: { tasks: [], count: 0 },
      ToDo: { tasks: [], count: 0 },
      WaitingForUser: { tasks: [], count: 0 },
      InProgress: { tasks: [], count: 0 },
      Done: { tasks: [], count: 0 },
      Failed: { tasks: [], count: 0 },
    }
  }
  const tasksGroupsByStatus = ref<GroupedTasks>(getDefaultTasksGroupsByStatus())

  const listRootTasks = async (params: ListTasksParams): Promise<void> => {
    try {
      const rootTasks = await listRootTasksReq(params)
      rootTasks.forEach((task) => {
        if (!tasks.value.find((a) => a.id === task.id)) {
          tasks.value.push(task)
        }
      })
    } catch (error) {
      errorToast(String(error))
    }
  }
  const selectedGroupCount = computed(() => {
    if (selectedGroup.value) {
      return tasksGroupsByStatus.value[selectedGroup.value].count
    }
    return 0
  })

  const pageSize = ref(20)
  const { currentPage, totalPages, setPage } = usePagination({
    count: selectedGroupCount,
    pageSize,
  })

  const listRootTasksByStatus = async (): Promise<void> => {
    try {
      const allStatuses = Object.values(TaskStatus)
      const statuses = selectedGroup.value ? [selectedGroup.value] : allStatuses

      const tasksBySelectedStatuses = statuses.map((status) =>
        listRootTasksByStatusReq({ status, pagination: { page: currentPage.value, per_page: pageSize.value } }),
      )

      const tasksByStatus = await Promise.all(tasksBySelectedStatuses)

      tasksGroupsByStatus.value = tasksByStatus.reduce((acc, curr) => {
        acc[curr.status] = { tasks: curr.tasks, count: curr.count }
        return acc
      }, {} as GroupedTasks)
    } catch (error) {
      errorToast(String(error))
    }
  }

  const updateTaskInGroup = (task: Task) => {
    // TODO null could be recieved here someway after Plan Execution action
    if (!task) {
      return null
    }
    const taskGroup = tasksGroupsByStatus.value[task.status]

    // Check if task is already in the group
    const oldTask = Object.values(tasksGroupsByStatus.value)
      .map((a) => a.tasks)
      .flat()
      .find((a) => a.id === task.id)

    // If the task is already in the group, update it
    if (oldTask) {
      const oldGroup = tasksGroupsByStatus.value[oldTask.status]

      const index = oldGroup.tasks.findIndex((a) => a.id === oldTask.id)

      // If the task status didn't change, just update the task
      if (oldTask.status === task.status) {
        oldGroup.tasks[index] = task
      } else {
        // If the task status changed, remove the old task and add the new one
        if (index !== undefined && index !== -1) {
          oldGroup.tasks.splice(index, 1)
          oldGroup.count -= 1
        }
        if (taskGroup.tasks.length === pageSize.value) {
          taskGroup.tasks.pop()
        }
        taskGroup.tasks.unshift(task)
        taskGroup.count += 1
      }
    } else {
      // If the task is not in the group, add it
      if (taskGroup.tasks.length === pageSize.value) {
        taskGroup.tasks.pop()
      }
      taskGroup.tasks.unshift(task)
      taskGroup.count += 1
    }
  }

  const createTask = async (task: CreateTask): Promise<void> => {
    try {
      const newTask = await createTaskReq(task)
      updateTaskInGroup(newTask)
      listRootTasksByStatus()
      selectTask(newTask.id)
    } catch (error) {
      errorToast(String(error))
    }
  }

  const duplicateTask = async (id: number): Promise<void> => {
    try {
      const task = await duplicateTaskReq(id)
      updateTaskInGroup(task)
      selectTask(task.id)
    } catch (error) {
      errorToast(String(error))
    }
  }

  const deleteTask = async (task: Task): Promise<void> => {
    try {
      await deleteTaskReq(task.id)
      await listRootTasksByStatus()
    } catch (error) {
      errorToast(String(error))
    }
  }
  const updateTask = async (task: UpdateTask): Promise<void> => {
    try {
      const updatedTask = await updateTaskReq(task)
      updateTaskInGroup(updatedTask)
    } catch (error) {
      errorToast(String(error))
    }
  }

  const reviseTask = async (id: number): Promise<void> => {
    try {
      const updatedTask = await reviseTaskReq(id)
      updateTaskInGroup(updatedTask)
      if (selectedTask.value?.id === id) {
        selectedTask.value = {
          ...updatedTask,
          children: selectedTask.value?.children || [],
        }
      }
    } catch (error) {
      errorToast(String(error))
    }
  }

  const executeTask = async (id: number): Promise<void> => {
    try {
      const updatedTask = await executeTaskReq(id)
      updateTaskInGroup(updatedTask)
    } catch (error) {
      errorToast(String(error))
    }
  }

  const planTask = async (id: number): Promise<void> => {
    try {
      const updatedTask = await planTaskReq(id)
      updateTaskInGroup(updatedTask)
    } catch (error) {
      errorToast(String(error))
    }
  }

  const taskUpdatedUnlisten = listen<Task>('tasks:updated', async (event) => {
    const task = event.payload

    if (!task.ancestry) {
      updateTaskInGroup(task)
    }
    if (task.id === selectedTask.value?.id) {
      selectedTask.value = {
        ...task,
        children: selectedTask.value?.children || [],
      }
    }
    if (getLastAncestor(task.ancestry) === selectedTask.value?.id) {
      const children = await listChildTasksReq(selectedTask.value.id)
      selectedTask.value.children = children || []
    }
    const { listChats, getById: getChatById } = useChatsStore()
    if (task.execution_chat_id && !getChatById(task.execution_chat_id)) {
      listChats()
    }
  }).catch((error) => {
    errorToast(String(error))
  })
  const tasksCreatedUnlisten = listen<Task>('tasks:created', async (event) => {
    const task = event.payload

    if (getLastAncestor(task.ancestry) === selectedTask.value?.id) {
      const children = await listChildTasksReq(selectedTask.value.id)
      selectedTask.value.children = children || []
    }
  }).catch((error) => {
    errorToast(String(error))
  })

  const $reset = async () => {
    tasksGroupsByStatus.value = getDefaultTasksGroupsByStatus()
    taskUpdatedUnlisten
    tasksCreatedUnlisten
  }

  return {
    $reset,
    tasks,
    tasksGroupsByStatus,
    selectedTask: readonly(selectedTask),
    selectedTaskQuery: readonly(selectedTaskQuery),
    selectedTaskParentId,
    selectTask,
    listRootTasks,
    createTask,
    deleteTask,
    updateTask,
    reviseTask,
    executeTask,
    planTask,
    duplicateTask,
    listRootTasksByStatus,
    isNewTask,
    isNewTaskQuery: readonly(isNewTaskQuery),
    setIsNewTask,
    selectedGroup,
    setGroup,
    totalPages,
    currentPage,
    setPage,
  }
})
