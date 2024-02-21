// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type ChatsList } from '../model'

export const listChats = async () => {
  const { chats } = await invoke<ChatsList>('list_chats')
  return chats
}
