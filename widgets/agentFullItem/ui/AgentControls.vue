<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAgentActions, useAgentsNavigation } from '~/features/agent'
  import { type Agent } from '~/entities/agents'
  import { BaseDropdown, BaseDropdownItem } from '~/shared/ui/dropdown'
  import { KebabIcon, CrossIcon } from '~/shared/ui/icons'

  const props = defineProps<{ agent: Agent }>()
  const emits = defineEmits<{ update: [agent: Agent] }>()
  const agentActions = useAgentActions(toRef(() => props.agent))
  const { setSelectedAgent } = useAgentsNavigation()

  const handleActionClick = (action: () => Promise<Agent | void>, isDelete = false) => {
    action().then(() => {
      if (isDelete) {
        navigateTo('/agents')
      }
    })
  }
</script>

<template>
  <div class="agent-controls">
    <BaseDropdown>
      <KebabIcon
        height="20"
        width="20"
      />
      <template #content>
        <BaseDropdownItem
          v-for="{ label, icon, action } in agentActions"
          :key="label"
          v-close-popper
          :class="['agent-controls__action', { delete: label === 'Delete Agent' }]"
          @click="handleActionClick(action, label === 'Delete Agent')"
        >
          <template #icon>
            <component
              :is="icon"
              class="agent-controls__action-icon"
            />
          </template>
          <template #label>
            <div class="agent-controls__action-label">
              {{ label }}
            </div>
          </template>
        </BaseDropdownItem>
      </template>
    </BaseDropdown>
    <CrossIcon
      width="20"
      height="20"
      @click="setSelectedAgent(null)"
    />
  </div>
</template>

<style lang="scss" scoped>
  .agent-controls {
    color: var(--text-tertiary);

    @include flex($gap: 16px, $align-items: center);
  }

  .agent-controls__action {
    &.delete {
      color: var(--status-failed);
    }

    &:hover {
      background-color: var(--surface-4);

      &:not(.delete) {
        color: var(--text-primary);
      }
    }

    @include font-inter-500(16px, 22px, var(--text-secondary));
    @include flex(row, flex-start, center, 8px);
  }
</style>
