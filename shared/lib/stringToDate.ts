// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import dayjs from 'dayjs'

export const stringToDate = (date: string) => {
  return dayjs(date).toDate()
}
