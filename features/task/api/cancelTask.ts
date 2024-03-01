// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Task } from '~/entities/tasks'

export const cancelTask = (id: number) => {
  return invoke<Task>('cancel_task', { id })
}
