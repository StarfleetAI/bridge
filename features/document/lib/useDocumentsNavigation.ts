// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { useRouteQuery } from '@vueuse/router'

export const useDocumentsNavigation = () => {
  const isCreateDocument = useRouteQuery('create', 'false', {
    transform: (value: 'false' | 'true') => value === 'true',
  })
  const enableCreateDocument = () => {
    isCreateDocument.value = true
    selectedDocument.value = null
  }
  const disableCreateDocument = () => {
    isCreateDocument.value = false
    selectedDocument.value = null
  }

  const selectedDocument = useRouteQuery('document', '', {
    transform: (value: string) => (isNaN(Number(value)) ? null : Number(value)),
  })
  const setSelectedDocument = (id: Nullable<number>) => {
    disableCreateDocument()
    selectedDocument.value = id
  }
  return {
    isCreateDocument: readonly(isCreateDocument),
    enableCreateDocument,
    disableCreateDocument,
    selectedDocument: readonly(selectedDocument),
    setSelectedDocument,
  }
}
