// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

interface Toast {
  color: string
  title: string
}
class ToastController {
  toasts: Toast[] = []

  add(toast: Toast, timeout = 3000) {
    this.toasts.push(toast)
    setTimeout(() => {
      this.remove(this.toasts.indexOf(toast))
    }, timeout)
  }

  remove(index: number) {
    this.toasts.splice(index, 1)
  }
}

export const useNotifications = () => {
  const toastController = new ToastController()

  const successNotification = (message: string) => {
    toastController.add({
      color: 'green',
      title: message
    })
  }
  const errorNotification = (message: string) => {
    toastController.add({
      color: 'red',
      title: message
    })
  }

  return {
    errorNotification,
    successNotification
  }
}
