// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const simpleHash = (str: string): string => {
  let hash = 0
  if (str.length === 0) {
    return hash.toString()
  }

  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i)
    hash = (hash << 5) - hash + char
    hash = hash & hash
  }
  return hash.toString()
}
