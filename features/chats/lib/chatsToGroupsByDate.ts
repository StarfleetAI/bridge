// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import dayjs from 'dayjs'
import { type Chat } from '~/entities/chat'
import { type ChatsGroups, type ChatsPeriod } from '../model'

export const chatsToGroupsByDate = (chats: Chat[]): ChatsGroups => {
  const result = new Map<ChatsPeriod, Chat[]>()

  if (!chats) {
    return []
  }

  const today = dayjs()
  const oneWeekAgo = today.subtract(1, 'week').startOf('day')
  const oneMonthAgo = today.subtract(1, 'month').startOf('day')
  const oneYearAgo = today.subtract(1, 'year').startOf('day')

  for (const chat of chats) {
    let period: ChatsPeriod | undefined

    if (dayjs(chat.created_at).isSame(today, 'day')) {
      period = 'Today'
    } else if (dayjs(chat.created_at).isAfter(oneWeekAgo)) {
      period = 'Previous 7 Days'
    } else if (dayjs(chat.created_at).isAfter(oneMonthAgo)) {
      period = 'Previous 30 Days'
    } else if (dayjs(chat.created_at).isAfter(oneYearAgo)) {
      period = dayjs(chat.created_at).format('MMMM') as ChatsPeriod
    } else {
      period = dayjs(chat.created_at).format('YYYY') as ChatsPeriod
    }

    if (!result.has(period)) {
      result.set(period, [])
    }

    result.get(period)?.push(chat)
  }

  for (const [key, value] of result.entries()) {
    if (value.length === 0) {
      result.delete(key)
    }
  }

  return Array.from(result)
}
