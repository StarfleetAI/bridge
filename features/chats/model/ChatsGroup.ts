// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Chat } from '~/entities/chat'
import { type Month } from '~/shared/model'

export type ChatsPeriod = 'Today' | 'Previous 7 Days' | 'Previous 30 Days' | Month | number

export type ChatsGroups = [ChatsPeriod, Chat[]][]
