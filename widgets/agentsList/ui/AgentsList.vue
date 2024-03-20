<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAgentsNavigation, useAgentsStore } from '~/features/agent'
  import { AgentItem } from '~/entities/agents'
  import type { Agent } from '~/entities/agents'

  defineProps<{
    agents: Agent[]
  }>()

  const { enableAgent } = useAgentsStore()

  const { setSelectedAgent, selectedAgent } = useAgentsNavigation()
  const setEnableAgent = (id: number, enabled: boolean) => {
    enableAgent(id, enabled)
  }
</script>

<template>
  <div class="agents-list">
    <AgentItem
      v-for="agent in agents"
      :key="agent.id"
      :agent="agent"
      :class="{ selected: agent.id === selectedAgent }"
      @click="setSelectedAgent(agent.id)"
      @toggle-enable="(enabled) => setEnableAgent(agent.id, enabled)"
    />
  </div>
</template>
<style scoped lang="scss">
  .agents-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 12px;
    padding: 16px 24px;
  }
</style>
