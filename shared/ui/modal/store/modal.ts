// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { defineStore } from 'pinia'

type ModalProps = Record<string, unknown>

type ModalComponent = Component | string

type OnCloseCallback<T = unknown> = ((returnValue?: T) => void) | null

export const useModalStore = defineStore('modal', () => {
  const isVisible = ref<boolean>(false)
  const contentComponent = ref<ModalComponent | null>(null)
  const contentProps = ref<ModalProps>({})
  const onCloseCallback = ref<OnCloseCallback>(null)

  function showModal(component: ModalComponent, props: ModalProps = {}, onClose?: OnCloseCallback) {
    contentComponent.value = component
    contentProps.value = props
    onCloseCallback.value = onClose || null
    isVisible.value = true
  }

  function closeModal<T = unknown>(returnValue?: T) {
    if (onCloseCallback.value) {
      onCloseCallback.value(returnValue)
    }
    isVisible.value = false
    contentComponent.value = null
    contentProps.value = {}
    onCloseCallback.value = null
  }

  return { isVisible, contentComponent, contentProps, showModal, closeModal }
})
