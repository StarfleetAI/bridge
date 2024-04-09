// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Chat } from '~/entities/chat'

export const getChat = async (id: number) => {
  return useInvoke<Chat>({ cmd: 'get_chat', args: { id } })
}
