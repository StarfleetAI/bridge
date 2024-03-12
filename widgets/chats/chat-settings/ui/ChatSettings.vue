<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAgentsNavigation } from '~/features/agent'
  import type { Agent } from '~/entities/agents'
  import { BridgeLargeIcon } from '~/shared/ui/icons'

  defineProps<{
    agent: Agent
  }>()
  const emits = defineEmits<{
    'change-agent': [agentId: number]
  }>()

  const { setSelectedAgent } = useAgentsNavigation()
</script>

<template>
  <div class="chat-settings">
    <div
      class="chat-settings__agent-wrapper"
      @click="setSelectedAgent(agent.id)"
    >
      <BridgeLargeIcon />
      <div class="agent__name">
        {{ agent.name }}
      </div>
      <div
        v-if="agent.description"
        class="agent__description"
      >
        {{ agent.description }}
      </div>
      <div class="agent__author">by StarfleetAI</div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .chat-settings {
    margin: auto;

    @include flex(column, center, center, 16px);
  }

  .chat-settings__agent-wrapper {
    display: contents;
  }

  .agent__name {
    @include font-inter-700(20px, 28px, var(--text-secondary));
  }

  .agent__description {
    text-align: center;

    @include font-inter-400(16px, 22px, var(--text-secondary));
  }

  .agent__author {
    @include font-inter-400(12px, 17px, var(--text-tertiary));
  }

  .agent__change {
    padding: 4px 8px;
    border: 1px solid var(--border-2);
    border-radius: 4px;

    @include font-inter-400(12px, 17px, var(--text-tertiary));
  }
</style>
