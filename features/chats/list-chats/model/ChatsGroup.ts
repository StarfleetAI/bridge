import type { Chat } from '~/entities/chat'

export type ChatsGroups = ['today' | 'lastWeek' | 'lastMonth', Chat[]][]
