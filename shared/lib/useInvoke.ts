// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { InvokeArgs } from '@tauri-apps/api/tauri'
import { toast } from 'vue3-toastify'
import 'vue3-toastify/dist/index.css'

// TODO replace direct calls of invoke in app with useInvoke
interface ExtendedInvoke<T = unknown, E = unknown> {
  cmd: string
  args?: InvokeArgs
  onSuccess?: (data?: T) => void
  onError?: (error: E) => void
  instantCall?: boolean
}
export const useInvoke = async <T = unknown, E = unknown>({
  cmd,
  args,
  onSuccess,
  onError,
  instantCall = true,
}: ExtendedInvoke<T, E>) => {
  const error = ref<E>()
  const data = ref<T>()
  const isLoading = ref(false)

  const execute = async () => {
    try {
      isLoading.value = true
      data.value = await invoke<T>(cmd, args)

      if (onSuccess) {
        onSuccess(data.value)
      }
    } catch (e) {
      error.value = e as E
      if (onError) {
        onError(error.value)
      }
      toast(`Error: ${error.value}`, {
        theme: 'dark',
        type: 'error',
        dangerouslyHTMLString: false,
      })
    } finally {
      isLoading.value = false
    }
  }

  if (instantCall) {
    await execute()
  }

  return { data, error, isLoading, execute }
}
