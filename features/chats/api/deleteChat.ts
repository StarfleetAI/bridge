// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const deleteChat = (id: number) => {
  return invoke('delete_chat', { id })
}
