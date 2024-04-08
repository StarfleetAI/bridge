<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import type { Agent } from '~/entities/agents'
  import { BaseDropdown, BaseDropdownItem } from '~/shared/ui/dropdown'
  import { BridgeSmallIcon, ChevronDownIcon } from '~/shared/ui/icons'

  withDefaults(
    defineProps<{
      agents: Agent[]
    }>(),
    {
      agents: () => [],
    },
  )
  const agent = defineModel<Agent>()
</script>

<template>
  <BaseDropdown>
    <div class="selected-agent">
      <ChevronDownIcon
        width="16"
        height="16"
        color="var(--text-secondary)"
      />
      <div class="selected-agent__name">
        {{ agent?.name }}
      </div>
      <BridgeSmallIcon />
    </div>
    <template #content>
      <BaseDropdownItem
        v-for="agentItem in agents"
        v-close-popper
        class="list-item"
        @click="agent = agentItem"
      >
        <template #icon>
          <BridgeSmallIcon />
        </template>
        <template #label>
          <div class="list-item__name">
            {{ agentItem.name }}
          </div>
        </template>
      </BaseDropdownItem>
    </template>
  </BaseDropdown>
</template>

<style lang="scss" scoped>
  .selected-agent {
    @include flex($gap: 8px, $align: center);
  }

  .selected-agent__name {
    @include font-inter-500(14px, 20px, var(--text-secondary));
  }

  .list-item {
    &:hover {
      background-color: var(--surface-4);
      color: var(--text-primary);
    }
  }

  .list-item__name {
    @include font-inter-500(16px, 22px, var(--text-primary));
  }
</style>
