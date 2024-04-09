// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const deleteChat = (id: number) => {
  return useInvoke({ cmd: 'delete_chat', args: { id } })
}
