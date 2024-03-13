// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const updateChatModelFullName = async (id: number, modelFullName: string) => {
  return invoke('change_chat_model_full_name', { id, modelFullName })
}
