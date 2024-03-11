// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type Document } from '~/entities/documents'
import { DeleteIcon } from '~/shared/ui/icons'

export const useDocumentActions = (document: Ref<Document>) => {
  const duplicateDocument = computed(() => {
    return {
      label: 'Duplicate Document',
      icon: DeleteIcon,
      action: () => {
        console.log(document)
      },
    }
  })

  const deleteDocument = computed(() => {
    return {
      label: 'Delete Document',
      icon: DeleteIcon,
      action: () => {
        console.log(document)
      },
    }
  })

  const baseActions = computed(() => {
    return [duplicateDocument.value, deleteDocument.value]
  })

  return computed(() => baseActions.value)
}
