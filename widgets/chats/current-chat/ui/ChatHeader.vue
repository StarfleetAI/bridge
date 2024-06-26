<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAgentsNavigation } from '~/features/agent'
  import { useChatsNavigation, useChatsStore } from '~/features/chats'
  import type { Agent } from '~/entities/agents'
  import type { Chat } from '~/entities/chat'
  import { BRIDGE_AGENT_ID } from '~/shared/lib'
  import { PinIcon, UnpinIcon, BridgeSmallIcon, SettingsIcon, NoAvatarIcon } from '~/shared/ui/icons'

  const props = defineProps<{
    agent: Agent
    chat: Nullable<Chat>
  }>()
  const chatsStore = useChatsStore()

  const handleClick = () => {
    if (props.chat) {
      chatsStore.toggleIsPinned(props.chat.id)
    }
  }
  const CurrentIcon = computed(() => (props.chat?.is_pinned ? UnpinIcon : PinIcon))
  const { setIsSettingsOpened } = useChatsNavigation()
  const { setSelectedAgent } = useAgentsNavigation()

  const handleClickSettings = () => {
    setSelectedAgent(null)
    setIsSettingsOpened(true)
  }
  const handleClickAgent = () => {
    setSelectedAgent(props.agent.id)
    setIsSettingsOpened(false)
  }
</script>

<template>
  <div class="chat-header">
    <component
      :is="CurrentIcon"
      v-if="chat"
      class="chat-header__pin"
      width="16"
      height="16"
      @click="handleClick"
    />
    <div
      class="chat-header__agent"
      @click="handleClickAgent"
    >
      {{ agent.name }}

      <BridgeSmallIcon v-if="agent.id === BRIDGE_AGENT_ID" />
      <NoAvatarIcon
        v-else
        width="24px"
        height="24px"
      />
    </div>
    <SettingsIcon
      class="chat-header__settigns"
      color="var(--text-tertiary)"
      @click="handleClickSettings"
    />
  </div>
</template>

<style lang="scss" scoped>
  .chat-header {
    flex-shrink: 0;
    width: 100%;
    height: 56px;
    padding: 8px 16px;
    border-bottom: 1px solid var(--border-3);
    background-color: var(--surface-1);

    @include font-inter-700(14px, 20px, var(--text-secondary));
    @include flex(row, space-between, center, 24px);
  }

  .chat-header__agent {
    margin-left: auto;

    @include flex(row, center, center, 8px);
    @include font-inter-500(14px, 20px, var(--text-secondary));
  }
</style>
