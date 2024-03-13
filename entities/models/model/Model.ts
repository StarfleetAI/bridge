// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0
import { type Provider } from './Provider'
export interface Model {
  // Provider of the model
  provider: Provider
  // Name of the model (e.g. `gpt-4-turbo-preview`)
  name: string
  // Context window size
  context_length: number
  // Maximum new tokens model can generate
  max_tokens: number
  // If model can take text input
  text_in: boolean
  // If model can generate text output
  text_out: boolean
  // If model can take image input
  image_in: boolean
  // If model can generate image output
  image_out: boolean
  // If model can take audio input
  audio_in: boolean
  // If model can generate audio output
  audio_out: boolean
  // Base URL for the model's API. Leave empty to use provider's default
  api_url?: string
  // API key for the API. Leave empty to use provider's default
  api_key?: string
  // If model is managed by Bridge
  is_system: boolean
  created_at: string
  updated_at: string
}
