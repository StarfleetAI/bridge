import type { Task } from './Task'

export interface SelectedTask extends Task {
  children: Task[]
}
