export const deleteTask = (id: number) => {
  return invoke('delete_task', { id })
}
