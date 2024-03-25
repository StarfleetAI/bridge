// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const getTaskResultTextData = (id: number): Promise<string> => {
  return invoke<string>('get_task_result_text_data', { id })
}
