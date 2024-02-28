export const reviseTask = (id: number) => {
  return invoke('revise_task', { id })
}
