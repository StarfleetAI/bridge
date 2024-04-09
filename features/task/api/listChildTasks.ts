// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type TasksList } from '../model'

export const listChildTasks = async (id: number) => {
  return useInvoke<TasksList>({ cmd: 'list_child_tasks', args: { id } })
}
