// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const toKebabCase = (str: string) => {
  return str
    .replace(/[^a-zA-Z0-9]+/g, '-')
    .replace(/-+/g, '-')
    .replace(/^-|-$/g, '')
    .toLowerCase()
}
