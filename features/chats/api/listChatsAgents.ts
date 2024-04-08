// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { ChatsAgents } from '../model'

export const listChatsAgents = () => {
  return useInvoke<ChatsAgents>({ cmd: 'list_agents_chats' })
}
