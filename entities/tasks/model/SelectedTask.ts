// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Task } from './Task'

export interface SelectedTask extends Task {
  children: Task[]
}
