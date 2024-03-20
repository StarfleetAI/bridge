// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { useRouteQuery } from '@vueuse/router'

export const useTasksNavigation = () => {
  const isCreateTask = useRouteQuery('create', null, { transform: (value: null | 'true') => value === 'true' || null })
  const enableCreateTask = () => {
    isCreateTask.value = true
    selectedTask.value = null
  }
  const disableCreateTask = () => {
    isCreateTask.value = null
  }

  const selectedTask = useRouteQuery('task', '', {
    transform: (value: string) => (isNaN(Number(value)) ? null : Number(value)),
  })

  const setSelectedTask = (id: Nullable<number>) => {
    disableCreateTask()
    selectedTask.value = id
  }

  return {
    isCreateTask: readonly(isCreateTask),
    enableCreateTask,
    disableCreateTask,
    selectedTask: readonly(selectedTask),
    setSelectedTask,
  }
}
