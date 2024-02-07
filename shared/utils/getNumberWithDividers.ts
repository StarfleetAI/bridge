// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const getNumberWithDividers = (value?: number) => {
  if (value) {
    return value.toLocaleString('en-US')
  }

  return ''
}
