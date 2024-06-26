// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Chat } from '~/entities/chat'
import { type CreateChat } from '../model'

export const createChat = (request: CreateChat) => {
  return useInvoke<Chat>({ cmd: 'create_chat', args: { request } })
}
