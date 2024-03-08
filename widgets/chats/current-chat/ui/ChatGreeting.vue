<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import type { Agent } from '~/entities/agents'
  import { BridgeLargeIcon } from '~/shared/ui/icons'
  import { useModalStore } from '~/shared/ui/modal'

  defineProps<{
    agent: Agent
  }>()
  const emits = defineEmits<{
    'change-agent': [agentId: number]
  }>()

  const { showModal } = useModalStore()
  const AgentsModal = defineAsyncComponent(() => import('./ChangeAgentModal.vue'))
  const openModal = () => {
    showModal(AgentsModal, {}, (val) => {
      if (val) {
        emits('change-agent', val as number)
      }
    })
  }
</script>

<template>
  <div class="greeting">
    <BridgeLargeIcon />
    <div class="agent-name">
      {{ agent.name }}
    </div>
    <div
      v-if="agent.description"
      class="agent-description"
    >
      {{ agent.description }}
    </div>
    <div class="agent__author">by StarfleetAI</div>
    <div
      class="agent__change"
      @click="openModal"
    >
      Change Agent
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .greeting {
    height: 100%;

    @include flex(column, center, center, 16px);
  }

  .agent-name {
    @include font-inter-700(20px, 28px, var(--text-secondary));
  }

  .agent__description {
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
