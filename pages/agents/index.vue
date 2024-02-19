<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { AbilitiesList } from '~/widgets/abilitiesList'
  import { AbilityFullItem } from '~/widgets/abilityFullItem'
  import { AgentFullItem } from '~/widgets/agentFullItem'
  import { AgentsList } from '~/widgets/agentsList'
  import { AgentIcon, AbilitiesIcon, LibraryIcon, StoreIcon } from '~/shared/icons'
  import { ToggleSwitch } from '~/shared/ui/toggle-switch'

  definePageMeta({
    title: 'Agents'
  })

  const entity: Ref<string> = ref('agents')
</script>

<template>
  <div class="main-content">
    <div class="main-content__header">
      <ToggleSwitch v-model="entity">
        <template #option-agents>
          <AgentIcon :color="entity === 'agents' ? 'var(--text-primary)' : 'var(--text-tertiary)'" /> Agents
        </template>
        <template #option-abilities>
          <AbilitiesIcon :color="entity === 'abilities' ? 'var(--text-primary)' : 'var(--text-tertiary)'" /> Abilities
        </template>
      </ToggleSwitch>
    </div>
    <div v-if="entity === 'agents'">
      <div class="list-title"><LibraryIcon /> Library <span>4</span></div>
      <AgentsList />
      <div class="list-title"><StoreIcon /> Store <span>4</span></div>
      <AgentsList />
    </div>
    <div v-if="entity === 'abilities'">
      <div class="list-title"><LibraryIcon /> Library <span>4</span></div>
      <AbilitiesList />
      <div class="list-title"><StoreIcon /> Store <span>4</span></div>
      <AbilitiesList />
    </div>
  </div>
  <div class="side-content">
    <AgentFullItem v-if="entity === 'agents'" />
    <AbilityFullItem v-if="entity === 'abilities'" />
  </div>
</template>

<style lang="scss" scoped>
  div {
    color: var(--text-primary);
  }

  .main-content {
    width: 60%;
    min-height: calc(100vh - 44px);
    padding: 20px;
  }

  .side-content {
    overflow-y: auto;
    width: 40%;
    min-height: calc(100vh - 44px);
    background: var(--side-panel);
  }

  .main-content__header {
    @include flex(row, space-between, center);
  }

  .list-title {
    gap: 8px;
    margin: 24px 0;

    span {
      @include font-inter-400(16px, 22px, var(--text-tertiary));
    }

    @include font-inter-700(16px, 22px, var(--text-secondary));
    @include flex(row, start, center);
  }
</style>
