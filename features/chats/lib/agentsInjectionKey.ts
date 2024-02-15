// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import type { InjectionKey, DeepReadonly } from 'vue'
import { type Agent } from '~/entities/agent'

export const agentsInjectionKey = Symbol('agents list') as InjectionKey<DeepReadonly<Ref<Agent[]>>>
