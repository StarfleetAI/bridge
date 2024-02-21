// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { Marked } from 'marked'

export const getMarkdown = (text: string) => {
  const marked = new Marked()

  return marked.parse(text)
}
