// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export const listDocuments = async () => {
  return [
    { id: 1, title: 'Document 1', created_at: new Date(), updated_at: new Date() },
    { id: 2, title: 'Document 2', created_at: new Date(), updated_at: new Date() },
  ]
}
