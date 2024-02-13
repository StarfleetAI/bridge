import type { AbilitiesList } from '../model'

export const listAbilities = async () => {
  const { abilities } = await invoke<AbilitiesList>('list_abilities')

  return abilities
}
