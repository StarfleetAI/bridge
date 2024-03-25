// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { TaskResultKind, type TaskResults } from '~/entities/tasks'
import { getTaskResultTextData } from './getTaskResultTextData'

export const getTaskResults = async (taskId: number): Promise<TaskResults> => {
  let results = await invoke<TaskResults>('list_task_results', { taskId })
  const textResults = results.filter((result) => result.kind === TaskResultKind.Text)
  if (textResults.length) {
    const resultIds = textResults.map((result) => result.id)
    const resultData = await Promise.all(resultIds.map((id) => getTaskResultTextData(id)))

    const resultDataMap = Object.fromEntries(resultIds.map((id, i) => [id, resultData[i]]))
    results = results.map((result) => ({
      ...result,
      parsed_data: resultDataMap[result.id],
    }))
  }

  return results
}
