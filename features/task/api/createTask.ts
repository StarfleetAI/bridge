// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Task } from '~/entities/tasks'
import { type CreateTask } from '../model'

export const createTask = (request: CreateTask) => {
  return invoke<Task>('create_task', { request })
}
