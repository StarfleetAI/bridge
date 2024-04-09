// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Task } from '~/entities/tasks'

export const reviseTask = (id: number) => {
  return useInvoke<Task>({ cmd: 'revise_task', args: { id } })
}
