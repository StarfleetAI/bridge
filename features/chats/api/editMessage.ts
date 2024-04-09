// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Message } from '~/entities/chat'

export const editMessage = (request: { id: number; content: string }) => {
  return useInvoke<Message>({ cmd: 'update_message_content', args: { ...request } })
}
