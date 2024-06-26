// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export interface BridgeEvent<T> {
  event: string
  data: T
}
