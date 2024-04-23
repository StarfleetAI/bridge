// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export interface Ability {
  id: number
  name: string
  description: string
  code: string
  created_at: Date
  updated_at: Date
  parameters_json: Definition
}

export interface Definition {
  name: string
  parameters: ParametersJson
}

export interface ParametersJson {
  type: string
  properties: Properties
}

export type Properties = Record<string, Property>

export interface Property {
  type: string
  description: string
}
