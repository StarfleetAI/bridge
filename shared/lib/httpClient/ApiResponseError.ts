// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { ErrorStatus } from './ErrorStatus'

export class ApiResponseError<T> {
  body: T

  status: ErrorStatus

  text: string

  constructor(response: ApiResponseError<T>) {
    Object.assign(this, response)
  }
}
