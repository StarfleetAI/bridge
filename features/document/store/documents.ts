// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Document } from '~/entities/documents'
import {
  listDocuments as listDocumentsReq,
  deleteDocument as deleteDocumentReq,
  createDocument as createDocumentReq,
} from '../api'
import { type CreateDocument } from '../model'

export const useDocumentsStore = defineStore('documents', () => {
  const documents = ref<Document[]>([])
  const getById = (id: Nullable<number | string>): Document | undefined => {
    return documents.value.find((a) => a.id === id)
  }
  const listDocuments = async () => {
    const agentsList = await listDocumentsReq()
    documents.value = agentsList
  }

  const createDocument = async (request: CreateDocument) => {
    await createDocumentReq(request)
    documents.value.push({ id: 1, ...request })
  }

  const deleteDocument = async (id: number): Promise<void> => {
    await deleteDocumentReq(id)
    const index = documents.value.findIndex((a) => a.id === id)
    if (index !== undefined && index !== -1) {
      documents.value.splice(index, 1)
    }
  }

  const $reset = () => {
    documents.value = []
  }

  return {
    documents,
    getById,
    listDocuments,
    createDocument,
    deleteDocument,
    $reset,
  }
})
