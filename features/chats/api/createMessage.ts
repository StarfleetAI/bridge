// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Message } from '~/entities/chat'
import { type CreateMessage } from '../model'

export const createMessage = async (request: CreateMessage) => {
  return invoke<Message>('create_message', { request })
}
