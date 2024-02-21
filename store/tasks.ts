// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { invoke } from '@tauri-apps/api/tauri'
import { defineStore } from 'pinia'

export enum Status {
  NEW = 'New',
  TODO = 'Todo',
  IN_PROGRESS = 'InProgress',
  WAITING_FOR_USER = 'WaitingForUser',
  PAUSED = 'Paused',
  DONE = 'Done',
  FAILED = 'Failed',
  CANCELED = 'Canceled',
}

export interface Task {
  id: number
  agent_id: number
  origin_chat_id: undefined | number
  control_chat_id: undefined | number
  execution_chat_id: undefined | number
  title: string
  summary: string
  status: Status
  ancestry: undefined | string
  ancestry_level: number
  created_at: Date
  updated_at: Date
}

export interface TasksList {
  tasks: Task[]
}

export interface CreateTask {
  agent_id: number
  title: string
  summary: string
  ancestry: undefined | string
}

export const useTasksStore = defineStore('tasks', {
  state: () => ({
    tasks: [] as Task[],
  }),

  getters: {
    getById:
      (state) =>
      (id: number | string | undefined): Task | undefined => {
        if (id === undefined) {
          return undefined
        }

        if (typeof id === 'string') {
          id = parseInt(id, 10)
        }

        return state.tasks.find((a) => a.id === id)
      },
  },

  actions: {
    async listRootTasks() {
      const tasks = await invoke<TasksList>('list_root_tasks')
      tasks.tasks.forEach((task) => {
        if (!this.tasks.find((a) => a.id === task.id)) {
          this.tasks.push(task)
        }
      })
    },

    async listChildTasks(id: number) {
      const tasks = await invoke<TasksList>('list_child_tasks', { id })
      tasks.tasks.forEach((task) => {
        if (!this.tasks.find((a) => a.id === task.id)) {
          this.tasks.push(task)
        }
      })
    },

    async createTask(task: CreateTask) {
      const newTask = await invoke<Task>('create_task', { task })
      this.tasks.push(newTask)
    },

    async getTask(id: number) {
      const task = await invoke<Task>('get_task', { id })
      this.tasks.push(task)
    },

    async deleteTask(id: number) {
      await invoke('delete_task', { id })
      const index = this.tasks.findIndex((a) => a.id === id)
      if (index !== undefined && index !== -1) {
        this.tasks.splice(index, 1)
      }
    },
  },
})
