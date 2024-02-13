import type { InjectionKey, DeepReadonly } from 'vue'
import { type Ability } from '~/entities/ability'

export const abilitiesInjectionKey = Symbol('abilities list') as InjectionKey<DeepReadonly<Ref<Ability[]>>>
