<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { AgentItem } from '~/entities/agents'
  import type { Agent } from '~/entities/agents'

  defineProps<{
    agents: Agent[]
  }>()
</script>

<template>
  <div class="agents-list">
    <AgentItem
      v-for="agent in agents"
      :key="agent.id"
      :agent="agent"
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
