// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type TaskStatus, type Task } from '~/entities/tasks'

export type TasksGroupName = 'Drafts' | 'To Do' | 'Waiting For User' | 'In Progress' | 'Done' | 'Failed'

export type GroupedTasks = Record<TaskStatus, Task[]>
