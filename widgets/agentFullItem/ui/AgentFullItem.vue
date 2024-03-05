<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { useAbilitiesStore } from '~/features/ability'
  import { useAgentsStore, useAgentsNavigation } from '~/features/agent'
  import { type Agent } from '~/entities/agents'
  import { AbilityIcon } from '~/shared/ui/icons'
  import AgentControls from './AgentControls.vue'

  const { selectedAgent } = useAgentsNavigation()

  const { abilities } = storeToRefs(useAbilitiesStore())

  const { getById } = useAgentsStore()

  const agent = ref(getById(selectedAgent.value!) as Agent)

  const agentAbilities = computed(() =>
    abilities.value.filter((ability) => agent.value.ability_ids.includes(ability.id)),
  )
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
    <div class="agent-full-item__abilities">
      <div class="agent-full-item__abilities-title"><AbilityIcon /> Abilities</div>
      <div class="agent-full-item__abilities-list">
        <div
          v-for="ability in agentAbilities"
          :key="ability.id"
          class="agent-full-item__abilities-list-item"
        >
          <div class="agent-full-item__abilities-list-item-name">{{ ability.name }}</div>
          <div class="agent-full-item__abilities-list-item-description">{{ ability.description }}</div>
        </div>
      </div>
    </div>
  </div>
</template>
<style scoped lang="scss">
  .agent-full-item {
    &__body {
      padding: 24px;
      border-bottom: 1px solid var(--border-3);

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

    &__abilities {
      padding: 24px;
    }

    &__abilities-title {
      padding: 0 0 24px;
      width: 100%;

      @include font-inter-500(14px, 20px, var(--text-secondary));
      @include flex(row, start, center, 4px);
    }

    &__abilities-list {
      width: 100%;

      @include flex(column);
    }

    &__abilities-list-item {
      padding: 8px 16px;
      border-radius: 6px;
      background: var(--surface-3);
      margin-bottom: 6px;

      @include flex(row, start, center);
    }

    &__abilities-list-item-name {
      margin-right: 8px;

      @include font-inter-500(14px, 20px, var(--text-primary));
    }

    &__abilities-list-item-description {
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
      flex: 1;

      @include font-inter-400(12px, 17px, var(--text-tertiary));
    }
  }
</style>
