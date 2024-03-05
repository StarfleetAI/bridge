<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { useAgentsStore, useAgentsNavigation } from '~/features/agent'
  import { type Agent } from '~/entities/agents'
  import AgentControls from './AgentControls.vue'

  const { selectedAgent } = useAgentsNavigation()

  const { getById } = useAgentsStore()

  const agent = ref(getById(selectedAgent.value!) as Agent)
</script>
<template>
  <div class="agent-full-item">
    <div class="agent-full-item__head">
      <div class="agent-full-item__title">Agent {{ agent.id }}</div>
      <AgentControls :agent="agent" />
    </div>
    <div class="agent-full-item__body">
      <div class="agent-full-item__avatar" />
      <div class="agent-full-item__name">{{ agent.name }}</div>
      <div class="agent-full-item__text">
        {{ agent.description }}
      </div>
      <div class="agent-full-item__info">by Alex Johnson • installed 4,322 times</div>
    </div>
  </div>
</template>
<style scoped lang="scss">
  .agent-full-item {
    &__body {
      padding: 24px;

      @include flex(column, start, center);
    }

    &__head {
      height: 57px;
      padding: 12px 24px;
      border-bottom: 1px solid var(--border-3);

      @include flex(row, space-between, center);
    }

    &__title {
      @include font-inter-700(14px, 20px, var(--text-secondary));
    }

    &__avatar {
      flex-shrink: 0;
      width: 80px;
      height: 80px;
      margin-bottom: 24px;
      border-radius: 50%;
      background: var(--text-secondary);
    }

    &__name {
      margin-bottom: 8px;

      @include font-inter-500(18px, 25px, var(--text-primary));
    }

    &__text {
      margin-bottom: 8px;

      @include font-inter-400(16px, 22px, var(--text-secondary));
    }

    &__info {
      @include font-inter-400(14px, 20px, var(--text-tertiary));
    }
  }
</style>
