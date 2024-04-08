// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { TaskStatus } from '~/entities/tasks'
import { type ListTasksByStatusParams, type TasksList } from '../model'

export const listRootTasksByStatus = async <T = TaskStatus>({ pagination, status }: ListTasksByStatusParams) => {
  const request = await useInvoke<TasksList<T> & { status: TaskStatus }>({
    cmd: 'list_root_tasks_by_status',
    args: { pagination, status },
  })
  if (request.data.value) {
    request.data.value = {
      ...request.data.value,
      status,
    }
  }
  return request
}
