import type { AgentsList } from '../model'

export const listAgents = async () => {
  const { agents } = await invoke<AgentsList>('list_agents')

  return agents
}
