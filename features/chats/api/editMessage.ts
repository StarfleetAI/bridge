// Copyright 2024 StarfleetAI

import type { Message } from '~/entities/chat'

// SPDX-License-Identifier: Apache-2.0
export const editMessage = (request: { id: number; content: string }): Promise<Message> => {
  return invoke<Message>('edit_message', { ...request })
}
