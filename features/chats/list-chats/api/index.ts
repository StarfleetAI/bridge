import { invoke } from '@tauri-apps/api/tauri'
import { chatsToGroupsByDate } from '../lib'
import { type ChatsList } from '../model'

export const listChats = async () => {
  const { chats } = await invoke<ChatsList>('list_chats')

  return chatsToGroupsByDate(chats)
}
