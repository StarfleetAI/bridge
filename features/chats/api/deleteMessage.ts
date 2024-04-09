// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const deleteMessage = (id: number) => {
  return useInvoke({ cmd: 'delete_message', args: { id } })
}
