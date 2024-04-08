// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const updateChatModelFullName = async (id: number, modelFullName: Nullable<string>) => {
  return useInvoke({ cmd: 'update_chat_model_full_name', args: { id, modelFullName } })
}
