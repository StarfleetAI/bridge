<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { AbilitiesList } from '~/widgets/abilitiesList'
  import { AgentsList } from '~/widgets/agentsList'
  import { useAgentsNavigation, useAgentsStore } from '~/features/agent'
  import { BaseContainer, BaseButton } from '~/shared/ui/base'
  import { LibraryIcon, StoreIcon, PlusIcon } from '~/shared/ui/icons'
  import { ToggleSwitch } from '~/shared/ui/toggle-switch'

  definePageMeta({
    title: 'Agents',
  })

  const { agents } = storeToRefs(useAgentsStore())

  const { isCreateAgent, enableCreateAgent, selectedAgent } = useAgentsNavigation()

  const entity: Ref<string> = ref('agents')

  const AgentFullItem = defineAsyncComponent(async () => {
    const module = await import('~/widgets/agentFullItem')
    return module.AgentFullItem
  })

  const AgentForm = defineAsyncComponent(async () => {
    const module = await import('~/widgets/agentForm')
    return module.AgentForm
  })

  const sideContentComponent = computed(() => {
    if (isCreateAgent.value) {
      return AgentForm
    }
    if (selectedAgent.value) {
      return AgentFullItem
    }
    return null
  })
</script>

<template>
  <BaseContainer>
    <template #main>
      <div class="main-content">
        <div class="main-content__header">
          <ToggleSwitch v-model="entity">
            <template #option-agents> Agents </template>
            <template #option-abilities> Abilities </template>
          </ToggleSwitch>
          <BaseButton
            :disabled="isCreateAgent"
            size="medium"
            class="task-list__create"
            @click="enableCreateAgent"
          >
            <template #icon>
              <PlusIcon />
            </template>
            Create new
          </BaseButton>
        </div>
        <div v-if="entity === 'agents'">
          <div class="list-title"><LibraryIcon /> Library <span>4</span></div>
          <AgentsList :agents="agents" />
        </div>
        <div v-if="entity === 'abilities'">
          <div class="list-title"><LibraryIcon /> Library <span>4</span></div>
          <AbilitiesList />
          <div class="list-title"><StoreIcon /> Store <span>4</span></div>
          <AbilitiesList />
        </div>
      </div>
    </template>
    <template #additional>
      <div class="side-content">
        <component :is="sideContentComponent" />
      </div>
    </template>
  </BaseContainer>
</template>

<style lang="scss" scoped>
  div {
    color: var(--text-primary);
  }

  .main-content {
    flex: 1;
  }

  .side-content {
    width: 100%;
    height: 100%;
    background: var(--surface-1);
    border-left: 1px solid var(--border-3);
  }

  .main-content__header {
    padding: 12px 24px 0;
    border-bottom: 1px solid var(--border-3);

    @include flex(row, space-between, center);
  }

  .list-title {
    gap: 8px;
    margin: 24px 24px 0;

    span {
      @include font-inter-400(16px, 22px, var(--text-tertiary));
    }

    @include font-inter-700(16px, 22px, var(--text-secondary));
    @include flex(row, start, center);
  }
</style>
