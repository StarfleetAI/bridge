// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

const DEFAULT_SYMBOLS = ['', 'k', 'M']

export function abbreviateNumber(num: number, digit = 1): string {
  const sign = num > 0
  const absNum = Math.abs(num)

  const tier = Math.floor(Math.log10(absNum) / 3)

  if (tier === 0) {
    return (sign ? '' : '-') + absNum.toString()
  }

  const suffix = DEFAULT_SYMBOLS[tier]

  if (!suffix) {
    throw new RangeError()
  }

  const scale = 10 ** (tier * 3)
  const scaled = absNum / scale
  const rounded = scaled.toFixed(digit)

  return (sign ? '' : '-') + rounded + suffix
}
