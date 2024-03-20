// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Task } from '~/entities/tasks'

export const duplicateTask = async (id: number) => {
  return invoke<Task>('duplicate_task', { id })
}
