// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { toast } from 'vue3-toastify'
import 'vue3-toastify/dist/index.css'

export const useToast = () => {
  const errorToast = (message: string) => {
    toast(`Error: ${message}`, {
      theme: 'dark',
      type: 'error',
      dangerouslyHTMLString: false,
    })
  }
  const successToast = (message: string) => {
    toast(message, {
      theme: 'dark',
      type: 'success',
      dangerouslyHTMLString: false,
    })
  }

  const warnToast = (message: string) => {
    toast(`Warning: ${message}`, {
      theme: 'dark',
      type: 'warning',
      dangerouslyHTMLString: false,
    })
  }

  return { errorToast, successToast, warnToast }
}
