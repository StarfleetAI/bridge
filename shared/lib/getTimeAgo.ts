// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'

dayjs.extend(relativeTime)

export const getTimeAgo = ({
  date,
  dateFormat = 'DD.MM.YYYY',
  fullUnit,
}: {
  date?: Date | string
  dateFormat?: string
  fullUnit?: boolean
}) => {
  const timeAgo = ref<string>('')

  const dateObj = dayjs(date)
  const units = {
    second: {
      short: 's',
      full: [' second', ' seconds'],
    },
    minute: {
      short: 'm',
      full: [' minute', ' minutes'],
    },
    hour: {
      short: 'h',
      full: [' hour', ' hours'],
    },
    day: {
      short: 'd',
      full: [' day', ' days'],
    },
  }

  const getUnit = (unit: keyof typeof units, amount: number = 1) => {
    if (fullUnit) {
      const enOrdinalRules = new Intl.PluralRules('en-US', { type: 'ordinal' })
      const rule = enOrdinalRules.select(amount)
      const word = units[unit].full[rule === 'one' ? 0 : 1]
      return `${amount} ${word} ago`
    }
    return `${amount}${units[unit].short} ago`
  }
  // Calculate the time difference in seconds and format the time ago string accordingly
  const update = () => {
    const now = dayjs()
    const diffInSeconds = now.diff(dateObj, 'second')

    if (!date) {
      timeAgo.value = ''
    } else if (diffInSeconds < 60) {
      timeAgo.value = getUnit('second', Math.max(1, Math.floor(diffInSeconds)))
    } else if (diffInSeconds < 3600) {
      timeAgo.value = getUnit('minute', Math.floor(diffInSeconds / 60))
    } else if (diffInSeconds < 86400) {
      timeAgo.value = getUnit('hour', Math.floor(diffInSeconds / 3600))
    } else if (diffInSeconds < 604800) {
      timeAgo.value = getUnit('day', Math.floor(diffInSeconds / 86400))
    } else {
      timeAgo.value = dateObj.format(dateFormat)
    }
  }

  watchEffect(update)

  update() // Initial update

  return timeAgo
}
