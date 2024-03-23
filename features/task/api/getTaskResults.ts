// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { TaskResults } from '~/entities/tasks'

export const getTaskResults = (taskId: number): Promise<TaskResults> => {
  return invoke<TaskResults>('get_task_results', { taskId })
}
