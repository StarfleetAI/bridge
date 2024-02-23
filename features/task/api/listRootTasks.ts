// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type ListTasksParams, type TasksList } from '../model'

export const listRootTasks = async ({ pagination }: ListTasksParams) => {
  const { tasks } = await invoke<TasksList>('list_root_tasks', { pagination })
  return tasks
}
