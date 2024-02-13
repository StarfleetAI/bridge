import dayjs from 'dayjs'
import { type Chat } from '~/entities/chat'
import { type ChatsGroups } from '../model'

export const chatsToGroupsByDate = (chats: Chat[]): ChatsGroups => {
  const result: Map<'today' | 'lastWeek' | 'lastMonth', Chat[]> = new Map()
    .set('today', [])
    .set('lastWeek', [])
    .set('lastMonth', [])

  if (!chats) {
    return []
  }

  for (const chat of chats) {
    if (dayjs().isSame(dayjs(chat.created_at), 'day')) {
      result.get('today')?.push(chat)
    } else if (dayjs(chat.created_at).isAfter(dayjs().subtract(1, 'week').startOf('day'))) {
      result.get('lastWeek')?.push(chat)
    } else if (dayjs(chat.created_at).isBefore(dayjs().subtract(1, 'week').startOf('day'))) {
      result.get('lastMonth')?.push(chat)
    }
  }

  for (const key of result.keys()) {
    if (result.get(key)?.length === 0) {
      result.delete(key)
    }
  }

  return Array.from(result)
}
