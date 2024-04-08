// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type MessagesList } from '../model'

export const listChatMessages = async (chat_id: number) => {
  return useInvoke<MessagesList>({ cmd: 'list_messages', args: { request: { chat_id } } })
}
