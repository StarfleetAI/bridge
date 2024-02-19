// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const deleteAgent = (id: number) => {
  return invoke('delete_agent', { id })
}
