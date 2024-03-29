// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Task } from '~/entities/tasks'
import { type StatusGroup } from './StatusGroup'

export type GroupedTasks = {
  Drafts: Task[]
  Active: Task[]
  Completed: Task[]
  [Symbol.iterator]: () => Iterator<[StatusGroup, Task[]]>
}
