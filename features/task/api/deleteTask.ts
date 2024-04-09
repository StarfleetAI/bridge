// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0
export const deleteTask = (id: number) => {
  return useInvoke<void>({ cmd: 'delete_task', args: { id } })
}
