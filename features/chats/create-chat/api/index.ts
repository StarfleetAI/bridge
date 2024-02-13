import { type Chat } from '~/entities/chat'
import { type CreateChat } from '../model'

export const createChat = (request: CreateChat): Promise<Chat> => {
  return invoke<Chat>('create_chat', { request })
}
