// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { defineStore } from 'pinia'

type ModalProps = Record<string, any>

export const useModalStore = defineStore('modal', () => {
  const isVisible = ref<boolean>(false)
  const contentComponent = ref<any>(null)
  const contentProps = ref<ModalProps>({})
  const onCloseCallback = ref<(() => void) | null>(null)

  function showModal(component: any, props: ModalProps = {}, onClose?: () => void) {
    contentComponent.value = component
    contentProps.value = props
    onCloseCallback.value = onClose || null
    isVisible.value = true
  }

  function closeModal() {
    if (onCloseCallback.value) {
      onCloseCallback.value()
    }
    isVisible.value = false
    contentComponent.value = null
    contentProps.value = {}
    onCloseCallback.value = null
  }

  return { isVisible, contentComponent, contentProps, showModal, closeModal }
})
