import { type Message } from '~/entities/chat'
import { type CreateMessage } from '../model'

export const createMessage = async (request: CreateMessage) => {
  return invoke<Message>('create_message', { request })
}
