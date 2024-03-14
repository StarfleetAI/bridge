// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { Dayjs } from 'dayjs'

export const utcToLocalTime = (createdAt: string): Dayjs => {
  let dateString = createdAt
  if (createdAt.at(-1) !== 'Z') {
    dateString += 'Z'
  }

  const dayjs = useDayjs()

  return dayjs(dateString)
}
