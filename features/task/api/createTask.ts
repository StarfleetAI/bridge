import { type Task } from '~/entities/tasks'
import { type CreateTask } from '../model'

export const createTask = (task: CreateTask) => {
  return invoke<Task>('create_task', { task })
}
