<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useChatsStore } from '~/features/chats'
  import type { Agent } from '~/entities/agents'
  import type { Chat } from '~/entities/chat'
  import { PinIcon, UnpinIcon, BridgeSmallIcon } from '~/shared/ui/icons'

  const props = defineProps<{
    agent: Agent
    chat: Nullable<Chat>
  }>()
  const chatsStore = useChatsStore()

  const handleClick = () => {
    if (props.chat) {
      chatsStore.toggleIsPinned(props.chat.id, props.chat.is_pinned)
    }
  }
  const CurrentIcon = computed(() => (props.chat?.is_pinned ? UnpinIcon : PinIcon))
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
    <div class="chat-header__agent">
      {{ agent.name }}

      <BridgeSmallIcon />
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .chat-header {
    width: 100%;
    height: 56px;
    padding: 8px 16px;
    border-bottom: 1px solid var(--border-3);
    background-color: var(--surface-1);

    @include font-inter-700(14px, 20px, var(--text-secondary));
    @include flex(row, space-between, center);
  }

  .chat-header__agent {
    margin-left: auto;

    @include flex(row, center, center, 8px);
    @include font-inter-500(14px, 20px, var(--text-secondary));
  }
</style>
