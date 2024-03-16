// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { listen } from '@tauri-apps/api/event'
// eslint-disable-next-line boundaries/element-types
import { useChatsStore } from '~/features/chats'
import { TaskStatus, type Task } from '~/entities/tasks'
import {
  listRootTasks as listRootTasksReq,
  listChildTasks as listChildTasksReq,
  createTask as createTaskReq,
  deleteTask as deleteTaskReq,
  updateTask as updateTaskReq,
  reviseTask as reviseTaskReq,
  cancelTask as cancelTaskReq,
  pauseTask as pauseTaskReq,
  executeTask as executeTaskReq,
} from '../api'
import { groupTasks } from '../lib'
import { type CreateTask, type ListTasksParams, type UpdateTask } from '../model'

export const useTasksStore = defineStore('tasks', () => {
  const tasks = ref<Task[]>([])
  const getById = (id: number | string): Task | undefined => {
    if (typeof id === 'string') {
      id = parseInt(id, 10)
    }

    return tasks.value.find((a) => a.id === id)
  }
  const tasksGroupsByStatus = computed(() => {
    return groupTasks(tasks.value)
  })

  const listRootTasks = async (params: ListTasksParams): Promise<void> => {
    const rootTasks = await listRootTasksReq(params)
    rootTasks.forEach((task) => {
      if (!tasks.value.find((a) => a.id === task.id)) {
        tasks.value.push(task)
      }
    })
  }

  const listChildTasks = async (id: number): Promise<void> => {
    const childTasks = await listChildTasksReq(id)
    childTasks.forEach((task) => {
      if (!tasks.value.find((a) => a.id === task.id)) {
        tasks.value.push(task)
      }
    })
  }

  const createTask = async (task: CreateTask): Promise<void> => {
    const newTask = await createTaskReq(task)
    tasks.value.unshift(newTask)
  }

  const duplicateTask = async ({ title, summary, agent_id }: Task): Promise<Task> => {
    const taskToDuplicate: CreateTask = {
      title,
      summary,
      agent_id,
      status: TaskStatus.DRAFT,
    }
    const newTask = await createTaskReq(taskToDuplicate)
    tasks.value.unshift(newTask)
    return newTask
  }

  const deleteTask = async (id: number): Promise<void> => {
    await deleteTaskReq(id)
    const index = tasks.value.findIndex((a) => a.id === id)
    if (index !== undefined && index !== -1) {
      tasks.value.splice(index, 1)
    }
  }

  const updateTask = async (task: UpdateTask): Promise<Task> => {
    const updatedTask = await updateTaskReq(task)
    const index = tasks.value.findIndex((a) => a.id === task.id)
    if (index !== undefined && index !== -1) {
      tasks.value[index] = updatedTask
    }
    return updatedTask
  }

  const reviseTask = async (id: number): Promise<Task> => {
    const updatedTask = await reviseTaskReq(id)
    const index = tasks.value.findIndex((a) => a.id === id)
    if (index !== undefined && index !== -1) {
      tasks.value[index] = updatedTask
    }
    return updatedTask
  }

  const cancelTask = async (id: number): Promise<Task> => {
    const updatedTask = await cancelTaskReq(id)
    const index = tasks.value.findIndex((a) => a.id === id)
    if (index !== undefined && index !== -1) {
      tasks.value[index] = updatedTask
    }
    return updatedTask
  }

  const pauseTask = async (id: number): Promise<Task> => {
    const updatedTask = await pauseTaskReq(id)
    const index = tasks.value.findIndex((a) => a.id === id)
    if (index !== undefined && index !== -1) {
      tasks.value[index] = updatedTask
    }
    return updatedTask
  }

  const executeTask = async (id: number): Promise<Task> => {
    const updatedTask = await executeTaskReq(id)
    const index = tasks.value.findIndex((a) => a.id === id)
    if (index !== undefined && index !== -1) {
      tasks.value[index] = updatedTask
    }
    return updatedTask
  }

  const taskUpdatedUnlisten = listen<Task>('tasks:updated', (event) => {
    const task = event.payload as Task
    const index = tasks.value.findIndex((a) => a.id === task.id)
    if (index !== undefined && index !== -1) {
      tasks.value[index] = task
    }
    const { listChats, getById: getChatById } = useChatsStore()
    if (task.execution_chat_id && !getChatById(task.execution_chat_id)) {
      listChats()
    }
  })

  const $reset = async () => {
    tasks.value = []
    taskUpdatedUnlisten
  }

  return {
    $reset,
    tasks,
    tasksGroupsByStatus,
    getById,
    listRootTasks,
    listChildTasks,
    createTask,
    deleteTask,
    updateTask,
    reviseTask,
    cancelTask,
    pauseTask,
    executeTask,
    duplicateTask,
  }
})
