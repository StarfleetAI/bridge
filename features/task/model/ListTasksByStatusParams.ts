// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { TaskStatus } from '~/entities/tasks'
import type { ListTasksParams } from './ListTasksParams'

export interface ListTasksByStatusParams extends ListTasksParams {
  status?: TaskStatus
}
