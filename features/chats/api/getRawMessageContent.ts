// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const getRawMessageContent = (id: number) => {
  return useInvoke<string>({ cmd: 'get_raw_message_content', args: { id } })
}
