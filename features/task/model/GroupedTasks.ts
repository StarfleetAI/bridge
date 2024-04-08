// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type TaskStatus } from '~/entities/tasks'
import type { TasksList } from './TasksList'

export type TasksGroupName = 'Drafts' | 'To Do' | 'Waiting For User' | 'In Progress' | 'Done' | 'Failed'

export type GroupedTasks = Record<TaskStatus, TasksList>
