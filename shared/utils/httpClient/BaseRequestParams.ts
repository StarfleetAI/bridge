// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

type HTTPMethod = 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE'

export type BaseRequestParams = {
  body?: null | unknown
  endpoint: string
  formData?: boolean
  method?: HTTPMethod
}
