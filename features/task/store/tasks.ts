// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { listen } from '@tauri-apps/api/event'
import { useRouteQuery } from '@vueuse/router'
// eslint-disable-next-line boundaries/element-types
import { useChatsStore } from '~/features/chats'
import type { BridgeEvent } from '~/entities/events'
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
      navigateTo({
        path: '/tasks',
        query: {
          ...route.query,
          create: 'true',
          task: null,
        },
      })
    } else {
      navigateTo({
        path: '/tasks',
        query: {
          ...route.query,
          create: 'false',
          task: null,
        },
      })
    }
  }
  const selectTask = async (id: Nullable<number>) => {
    try {
      isNewTask.value = false
      isNewTaskQuery.value = false
      if (id) {
        navigateTo({
          path: '/tasks',
          query: {
            ...route.query,
            task: id,
            create: 'false',
          },
        })
        const [task, children] = await Promise.all([getTask(id), listChildTasksReq(id)])
        if (task.data.value) {
          selectedTask.value = {
            ...task.data.value,
            children: children.data.value?.tasks || [],
          }
          selectedTaskQuery.value = task.data.value.id
        }
      } else {
        await navigateTo({
          path: '/tasks',
          query: {
            ...route.query,
            create: 'false',
            task: null,
          },
        })
        selectedTask.value = null
        selectedTaskQuery.value = null
      }
    } catch (error) {
      useToast().errorToast(String(error))
    }
  }
  const getDefaultTasksGroupsByStatus = () => {
    return {
      Draft: {
        tasks: [],
        count: 0,
      },
      ToDo: {
        tasks: [],
        count: 0,
      },
      WaitingForUser: {
        tasks: [],
        count: 0,
      },
      InProgress: {
        tasks: [],
        count: 0,
      },
      Done: {
        tasks: [],
        count: 0,
      },
      Failed: {
        tasks: [],
        count: 0,
      },
    }
  }
  const tasksGroupsByStatus = ref<GroupedTasks>(getDefaultTasksGroupsByStatus())

  const listRootTasks = async (params: ListTasksParams): Promise<void> => {
    const { data } = await listRootTasksReq(params)
    data.value?.tasks.forEach((task) => {
      if (!tasks.value.find((a) => a.id === task.id)) {
        tasks.value.push(task)
      }
    })
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
    const allStatuses = Object.values(TaskStatus)
    const statuses = selectedGroup.value ? [selectedGroup.value] : allStatuses

    const tasksBySelectedStatuses = statuses.map((status) =>
      listRootTasksByStatusReq({
        status,
        pagination: {
          page: currentPage.value,
          per_page: pageSize.value,
        },
      }),
    )

    const tasksByStatus = await Promise.all(tasksBySelectedStatuses)

    tasksGroupsByStatus.value = tasksByStatus.reduce((acc, curr) => {
      if (curr.data.value) {
        acc[curr.data.value.status] = {
          tasks: curr.data.value.tasks,
          count: curr.data.value.count,
        }
      }
      return acc
    }, {} as GroupedTasks)
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
    const { data: newTask } = await createTaskReq(task)
    if (newTask.value) {
      updateTaskInGroup(newTask.value)
      listRootTasksByStatus()
      selectTask(newTask.value.id)
    }
  }

  const duplicateTask = async (id: number): Promise<void> => {
    const { data: task } = await duplicateTaskReq(id)
    if (task.value) {
      updateTaskInGroup(task.value)
      selectTask(task.value.id)
    }
  }

  const deleteTask = async (task: Task): Promise<void> => {
    await deleteTaskReq(task.id)
    listRootTasksByStatus()
  }
  const updateTask = async (task: UpdateTask): Promise<void> => {
    const { data: updatedTask } = await updateTaskReq(task)
    if (updatedTask.value) {
      updateTaskInGroup(updatedTask.value)
    }
  }

  const reviseTask = async (id: number): Promise<void> => {
    const { data: updatedTask } = await reviseTaskReq(id)
    if (updatedTask.value) {
      updateTaskInGroup(updatedTask.value)
      if (selectedTask.value?.id === id) {
        selectedTask.value = {
          ...updatedTask.value,
          children: selectedTask.value?.children || [],
        }
      }
    }
  }

  const executeTask = async (id: number): Promise<void> => {
    const { data: updatedTask } = await executeTaskReq(id)
    if (updatedTask.value) {
      updateTaskInGroup(updatedTask.value)
    }
  }

  const planTask = async (id: number): Promise<void> => {
    const { data: updatedTask } = await planTaskReq(id)
    if (updatedTask.value) {
      updateTaskInGroup(updatedTask.value)
    }
  }

  const taskUpdatedUnlisten = listen<BridgeEvent<Task>>('tasks:updated', async (event) => {
    const task = event.payload.data

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
      const { data } = await listChildTasksReq(selectedTask.value.id)
      selectedTask.value.children = data.value?.tasks || []
    }
    const { listChats, getById: getChatById } = useChatsStore()
    if (task.execution_chat_id && !getChatById(task.execution_chat_id)) {
      listChats()
    }
  }).catch((error) => {
    useToast().errorToast(String(error))
  })
  const tasksCreatedUnlisten = listen<BridgeEvent<Task>>('tasks:created', async (event) => {
    const task = event.payload.data

    if (getLastAncestor(task.ancestry) === selectedTask.value?.id) {
      const { data } = await listChildTasksReq(selectedTask.value.id)
      selectedTask.value.children = data.value?.tasks || []
    }
  }).catch((error) => {
    useToast().errorToast(String(error))
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
