// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Task } from '~/entities/tasks'

export const getTask = (id: number) => {
  return useInvoke<Task>({ cmd: 'get_task', args: { id } })
}
