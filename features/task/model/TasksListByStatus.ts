// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { TasksList } from './TasksList'

export interface TasksListByStatus<T> extends TasksList<T> {
  status: T
}
