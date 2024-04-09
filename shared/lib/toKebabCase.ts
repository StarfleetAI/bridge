// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const toKebabCase = (str: string) => {
  return str
    .replace(/([a-z])([A-Z])/g, '$1 $2')
    .replace(/[\s_]+/g, '-')
    .toLowerCase()
}
