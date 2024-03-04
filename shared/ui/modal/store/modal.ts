// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { defineStore } from 'pinia'

type ModalProps = Record<string, never>

type ModalComponent = Component | string

export const useModalStore = defineStore('modal', () => {
  const isVisible = ref<boolean>(false)
  const contentComponent = ref<ModalComponent | null>(null)
  const contentProps = ref<ModalProps>({})
  const onCloseCallback = ref<(() => void) | null>(null)

  function showModal(component: ModalComponent, props: ModalProps = {}, onClose?: () => void) {
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
