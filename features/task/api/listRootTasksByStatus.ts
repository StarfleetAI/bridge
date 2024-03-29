// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { TaskStatus } from '~/entities/tasks'
import { type ListTasksByStatusParams, type TasksList } from '../model'

export const listRootTasksByStatus = async <T = TaskStatus>({ pagination, status }: ListTasksByStatusParams) => {
  const { tasks } = await invoke<TasksList<T>>('list_root_tasks_by_status', { pagination, status })
  return tasks
}
