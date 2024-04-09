// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const toSentenceCase = (str: string) => {
  const result = str
    .replace(/([A-Z])|_/g, (_, p1) => (p1 ? ` ${p1}` : ' '))
    .trim()
    .replace(/\s+/g, ' ')
    .toLowerCase()
  return result.replace(/(^\s*\w|[.!?]\s*\w)/g, (firstLetter) => {
    return firstLetter.toUpperCase()
  })
}
