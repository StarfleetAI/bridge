// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Message } from '~/entities/chat'

export const editMessage = (request: { id: number; content: string }): Promise<Message> => {
  return invoke<Message>('update_message_content', { ...request })
}
