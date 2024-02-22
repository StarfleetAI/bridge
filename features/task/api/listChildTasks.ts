import { type TasksList } from '../model'

export const listChildTasks = async (id: number) => {
  const { tasks } = await invoke<TasksList>('list_child_tasks', { id })
  return tasks
}
