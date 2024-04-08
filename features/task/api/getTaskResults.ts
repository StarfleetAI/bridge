// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { TaskResultKind, type TaskResults } from '~/entities/tasks'
import { getTaskResultTextData } from './getTaskResultTextData'

export const getTaskResults = async (taskId: number) => {
  const results = await useInvoke<TaskResults>({ cmd: 'list_task_results', args: { taskId } })
  const textResults = results.data.value?.filter((result) => result.kind === TaskResultKind.Text) || []
  if (textResults.length) {
    const resultIds = textResults.map((result) => result.id)
    const resultData = await Promise.all(resultIds.map((id) => getTaskResultTextData(id)))

    const resultDataMap = Object.fromEntries(resultIds.map((id, i) => [id, resultData[i]]))
    results.data.value = results.data.value?.map((result) => ({
      ...result,
      parsed_data: resultDataMap[result.id].data.value,
    }))
  }

  return results
}
