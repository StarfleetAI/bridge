// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'

dayjs.extend(relativeTime)

export const getTimeAgo = ({
  date,
  shortDate,
  shortYear
}: {
  date?: Date | string
  shortDate?: boolean
  shortYear?: boolean
}) => {
  const timeAgo = ref<string>('')

  const dateObj = dayjs(date)

  const update = () => {
    const now = dayjs()
    const diffInSeconds = now.diff(dateObj, 'second')
    let dateFormat = 'DD.MM.YYYY'
    if (shortYear) {
      dateFormat = 'DD.MM.YY'
    } else if (shortDate) {
      dateFormat = 'MMM DD'
    }
    if (!date) {
      timeAgo.value = ''
    } else if (diffInSeconds < 60) {
      timeAgo.value = `${Math.max(1, Math.floor(diffInSeconds))}s ago`
    } else if (diffInSeconds < 3600) {
      timeAgo.value = `${Math.floor(diffInSeconds / 60)}m ago`
    } else if (diffInSeconds < 86400) {
      timeAgo.value = `${Math.floor(diffInSeconds / 3600)}h ago`
    } else if (diffInSeconds < 604800) {
      timeAgo.value = `${Math.floor(diffInSeconds / 86400)}d ago`
    } else if (dateObj.isSame(now, 'year')) {
      timeAgo.value = dateObj.format(dateFormat)
    } else {
      timeAgo.value = dateObj.format(dateFormat)
    }
  }

  watchEffect(update)

  update() // Initial update

  return timeAgo
}
