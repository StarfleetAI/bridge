// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Task } from '~/entities/tasks'

export const duplicateTask = async (id: number) => {
  return useInvoke<Task>({ cmd: 'duplicate_task', args: { id } })
}
