// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Task } from '~/entities/tasks'

export const planTask = (id: number) => {
  return invoke<Task>('plan_task', { id })
}
