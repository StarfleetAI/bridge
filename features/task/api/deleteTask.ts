// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0
export const deleteTask = (id: number) => {
  return invoke<void>('delete_task', { id })
}
