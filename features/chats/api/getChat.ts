// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Chat } from '~/entities/chat'

export const getChat = async (id: number) => {
  return invoke<Chat>('get_chat', { id })
}
