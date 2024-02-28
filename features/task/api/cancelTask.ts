export const cancelTask = (id: number) => {
  return invoke('cancel_task', { id })
}
