export const toggleIsPinned = (id: number) => {
  return invoke('toggle_chat_is_pinned', { id })
}
