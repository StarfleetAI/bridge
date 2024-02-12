// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export type FileType = "TXT" | "CSV";

export interface File {
  name: string
  type: FileType
  size?: number
  url?: string
  preview?: string
  created?: string
  modified?: string
  rows?: number
}
