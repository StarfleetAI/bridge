import { invoke } from '@tauri-apps/api/tauri'
import { type Chat } from '~/entities/chat'

export const getChat = async (id: number) => {
  return invoke<Chat>('get_chat', { id })
}
