// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type TaskStatus, type Task } from '~/entities/tasks'

export interface TasksList<T = TaskStatus> {
  tasks: Task<T>[]
}
