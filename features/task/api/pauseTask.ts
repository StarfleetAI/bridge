export const pauseTask = (id: number) => {
  return invoke('pause_task', { id })
}
