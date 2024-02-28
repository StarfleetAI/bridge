export const duplicateTask = (id: number) => {
  return invoke('duplicate_task', { id })
}
