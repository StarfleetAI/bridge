import type { Task } from '~/entities/tasks'

export const duplicateTask = async (id: number) => {
  return invoke<Task>('duplicate_task', { id })
}
