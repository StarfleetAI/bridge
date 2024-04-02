// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { listen } from '@tauri-apps/api/event'
// eslint-disable-next-line boundaries/element-types
import { useChatsStore } from '~/features/chats'
import { TaskStatus, type Task, type SelectedTask } from '~/entities/tasks'
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
  const selectedTaskParentId = computed(() => {
    const lastAncestor = selectedTask.value?.ancestry?.split('/').at(-1)
    if (lastAncestor) {
      return isNaN(Number(lastAncestor)) ? null : Number(lastAncestor)
    }
    return null
  })

  const isNewTask = ref(false)
  const setIsNewTask = (val: boolean) => {
    selectedTask.value = null
    isNewTask.value = val
    if (val) {
      navigateTo({ path: '/tasks', query: { create: 'true', task: null } })
    } else {
      navigateTo({ path: '/tasks', query: {} })
    }
  }
  const selectTask = async (id: Nullable<number>) => {
    isNewTask.value = false
    if (id) {
      navigateTo({ path: '/tasks', query: { task: id } })
      const [task, children] = await Promise.all([getTask(id), listChildTasksReq(id)])
      selectedTask.value = { ...task, children }
    } else {
      await navigateTo({ path: '/tasks', query: {} })
      selectedTask.value = null
    }
  }
  const tasksGroupsByStatus = ref<GroupedTasks>({
    Draft: [],
    ToDo: [],
    WaitingForUser: [],
    InProgress: [],
    Done: [],
    Failed: [],
  })
  const listRootTasks = async (params: ListTasksParams): Promise<void> => {
    const rootTasks = await listRootTasksReq(params)
    rootTasks.forEach((task) => {
      if (!tasks.value.find((a) => a.id === task.id)) {
        tasks.value.push(task)
      }
    })
  }

  const listRootTasksByStatus = async (): Promise<void> => {
    const statuses = Object.values(TaskStatus) as TaskStatus[]
    const tasksByStatus = (
      await Promise.all(
        statuses.map((status) => listRootTasksByStatusReq({ status, pagination: { page: 1, per_page: 16 } })),
      )
    ).flat()

    tasksGroupsByStatus.value = {
      Draft: tasksByStatus.filter((task) => task.status === TaskStatus.DRAFT),
      ToDo: tasksByStatus.filter((task) => task.status === TaskStatus.TODO),
      WaitingForUser: tasksByStatus.filter((task) => task.status === TaskStatus.WAITING_FOR_USER),
      InProgress: tasksByStatus.filter((task) => task.status === TaskStatus.IN_PROGRESS),
      Done: tasksByStatus.filter((task) => task.status === TaskStatus.DONE),
      Failed: tasksByStatus.filter((task) => task.status === TaskStatus.FAILED),
    }
  }

  const listChildTasks = async (id: number): Promise<void> => {
    await listChildTasksReq(id)
    // childTasks.forEach((task) => {
    //   if (!tasks.value.find((a) => a.id === task.id)) {
    //     tasks.value.push(task)
    //   }
    // })
  }

  const updateTaskInGroup = (task: Task) => {
    const taskGroup = tasksGroupsByStatus.value[task.status]

    // Check if task is already in the group
    const oldTask = Object.values(tasksGroupsByStatus.value)
      .flat()
      .find((a) => a.id === task.id)
    // If the task is already in the group, update it
    if (oldTask) {
      const oldGroup = tasksGroupsByStatus.value[oldTask.status]
      const index = oldGroup.findIndex((a) => a.id === oldTask.id)

      // If the task status didn't change, just update the task
      if (oldTask.status === task.status) {
        oldGroup[index] = task
      } else {
        // If the task status changed, remove the old task and add the new one
        if (index !== undefined && index !== -1) {
          oldGroup.splice(index, 1)
        }
        if (taskGroup.length === 16) {
          taskGroup.pop()
        }
        taskGroup.unshift(task)
      }
    } else {
      // If the task is not in the group, add it
      if (taskGroup.length === 16) {
        taskGroup.pop()
      }
      taskGroup.unshift(task)
    }
  }

  const createTask = async (task: CreateTask): Promise<Task> => {
    const newTask = await createTaskReq(task)
    updateTaskInGroup(newTask)

    return newTask
  }

  const duplicateTask = async (id: number): Promise<Task> => {
    const task = await duplicateTaskReq(id)
    updateTaskInGroup(task)

    return task
  }

  const deleteTask = async (task: Task): Promise<void> => {
    await deleteTaskReq(task.id)
    await listRootTasksByStatus()
  }
  const updateTask = async (task: UpdateTask): Promise<Task> => {
    const updatedTask = await updateTaskReq(task)
    updateTaskInGroup(updatedTask)

    return updatedTask
  }

  const reviseTask = async (id: number): Promise<Task> => {
    const updatedTask = await reviseTaskReq(id)
    updateTaskInGroup(updatedTask)

    return updatedTask
  }

  const executeTask = async (id: number): Promise<Task> => {
    const updatedTask = await executeTaskReq(id)
    console.log(updatedTask)
    updateTaskInGroup(updatedTask)

    return updatedTask
  }

  const planTask = async (id: number): Promise<Task> => {
    const updatedTask = await planTaskReq(id)
    updateTaskInGroup(updatedTask)

    return updatedTask
  }

  const taskUpdatedUnlisten = listen<Task>('tasks:updated', (event) => {
    const task = event.payload
    updateTaskInGroup(task)
    if (task.id === selectedTask.value?.id) {
      selectedTask.value = {
        ...task,
        children: selectedTask.value?.children || [],
      }
    }
    const { listChats, getById: getChatById } = useChatsStore()
    if (task.execution_chat_id && !getChatById(task.execution_chat_id)) {
      listChats()
    }
  })

  const $reset = async () => {
    tasksGroupsByStatus.value = {
      Draft: [],
      ToDo: [],
      WaitingForUser: [],
      InProgress: [],
      Done: [],
      Failed: [],
    }
    taskUpdatedUnlisten
  }

  return {
    $reset,
    tasks,
    tasksGroupsByStatus,
    selectedTask: readonly(selectedTask),
    selectedTaskParentId,
    selectTask,
    listRootTasks,
    listChildTasks,
    createTask,
    deleteTask,
    updateTask,
    reviseTask,
    executeTask,
    planTask,
    duplicateTask,
    listRootTasksByStatus,
    isNewTask,
    setIsNewTask,
  }
})
