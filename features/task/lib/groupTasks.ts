// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { TaskStatus, type Task } from '~/entities/tasks'
import { type StatusGroup, type GroupedTasks } from '../model'

export const groupTasks = (tasks: Task[]): GroupedTasks => {
  const getStatusGroup = (status: TaskStatus): StatusGroup => {
    switch (status) {
      case TaskStatus.DRAFT:
        return 'Drafts'
      case TaskStatus.DONE:
        return 'Completed'
      default: {
        return 'Active'
      }
    }
  }
  const groupedTasks: GroupedTasks = {
    Drafts: [],
    Active: [],
    Completed: [],
    // To keep the order always the same
    *[Symbol.iterator]() {
      const order: StatusGroup[] = ['Drafts', 'Active', 'Completed']
      for (const group of order) {
        yield [group, this[group]]
      }
    },
  }

  tasks.forEach((task) => {
    const statusGroup = getStatusGroup(task.status)
    groupedTasks[statusGroup].push(task)
  })
  // remove empty groups

  return groupedTasks
}
