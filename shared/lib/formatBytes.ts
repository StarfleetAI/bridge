// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const formatBytes = (bytes: number, decimals: number = 0): string => {
  if (bytes === 0) {
    return '0 B'
  }
  const factor = 1024
  const dm = decimals < 0 ? 0 : decimals
  const sizes = ['Bytes', 'kb', 'mb', 'gb']
  const sizeIndex = Math.floor(Math.log(bytes) / Math.log(factor))
  return `${parseFloat((bytes / factor ** sizeIndex).toFixed(dm))} ${sizes[sizeIndex]}`
}
