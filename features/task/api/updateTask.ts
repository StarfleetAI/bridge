// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Task } from '~/entities/tasks'
import { type UpdateTask } from '../model'

export const updateTask = (request: UpdateTask) => {
  return useInvoke<Task>({ cmd: 'update_task', args: { request } })
}
