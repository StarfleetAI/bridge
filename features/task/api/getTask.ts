import { type Task } from '~/entities/tasks'

export const getTask = (id: number) => {
  return invoke<Task>('get_task', { id })
}
