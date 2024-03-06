<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { AgentsList } from '~/widgets/agentsList'
  import { useAgentsNavigation, useAgentsStore } from '~/features/agent'
  import { BaseContainer, BaseButton } from '~/shared/ui/base'
  import { LibraryIcon, PlusIcon } from '~/shared/ui/icons'
  import { ToggleSwitch } from '~/shared/ui/toggle-switch'

  definePageMeta({
    title: 'Agents',
  })

  const entity = ref('agents')

  const { agents } = storeToRefs(useAgentsStore())

  const { isCreateAgent, enableCreateAgent, selectedAgent } = useAgentsNavigation()

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

  const createHandle = () => {
    enableCreateAgent()
  }
</script>

<template>
  <BaseContainer>
    <template #main>
      <div class="main-content">
        <div class="main-content__header">
          <ToggleSwitch
            v-model="entity"
            readonly
          >
            <template #option-agents>
              <div @click="() => navigateTo('/agents')">Agents</div>
            </template>
            <template #option-abilities>
              <div @click="() => navigateTo('/agents/abilities')">Abilities</div>
            </template>
          </ToggleSwitch>
          <BaseButton
            :disabled="isCreateAgent"
            size="medium"
            class="agents-list__create"
            @click="createHandle"
          >
            <template #icon>
              <PlusIcon />
            </template>
            Create new
          </BaseButton>
        </div>
        <div class="list-title"><LibraryIcon /> Library <span>4</span></div>
        <AgentsList :agents="agents" />
      </div>
    </template>
    <template
      v-if="sideContentComponent"
      #additional
    >
      <div class="side-content">
        <component
          :is="sideContentComponent"
          :key="String(selectedAgent)"
        />
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
    border-left: 1px solid var(--border-3);
    background: var(--surface-1);
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

  .agents-list__create {
    margin-top: -10px;
  }
</style>
