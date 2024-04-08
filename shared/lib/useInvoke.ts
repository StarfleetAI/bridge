// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { InvokeArgs } from '@tauri-apps/api/tauri'
import { toast } from 'vue3-toastify'
import 'vue3-toastify/dist/index.css'

// TODO replace direct calls of invoke in app with useInvoke
interface ExtendedInvoke<T, E = unknown> {
  cmd: string
  args?: InvokeArgs
  onSuccess?: (data: T) => void
  onError?: (error: E) => void
}
export const useInvoke = async <T = unknown>({ cmd, args, onSuccess, onError }: ExtendedInvoke<T>) => {
  try {
    const data = await invoke<T>(cmd, args)
    if (onSuccess) {
      onSuccess(data)
    }
    return data
  } catch (error) {
    if (onError) {
      onError(error)
    }
    toast(`Error: ${error}`, {
      theme: 'dark',
      type: 'error',
      dangerouslyHTMLString: false,
    })
    return error
  }
}
