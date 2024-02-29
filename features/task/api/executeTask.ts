// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Task } from '~/entities/tasks'

export const executeTask = (id: number) => {
  return invoke<Task>('execute_task', { id })
}
