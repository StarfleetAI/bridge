// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { useRouteQuery } from '@vueuse/router'

export const useTasksNavigation = () => {
  const isCreateTask = useRouteQuery('create', 'false', { transform: (value: 'false' | 'true') => value === 'true' })
  const enableCreateTask = () => {
    isCreateTask.value = true
    selectedTask.value = null
  }
  const disableCreateTask = () => {
    isCreateTask.value = false
  }

  const selectedTask = useRouteQuery('task', '', {
    transform: (value: string) => (isNaN(Number(value)) ? null : Number(value)),
  })

  const setSelectedTask = (id: Nullable<number>) => {
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
