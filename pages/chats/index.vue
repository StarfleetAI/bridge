<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { AgentFullItem } from '~/widgets/agentFullItem'
  import { ChatSettings } from '~/widgets/chats/chat-settings'
  import { ChatsHistory } from '~/widgets/chats/chats-history'
  import { CurrentChat } from '~/widgets/chats/current-chat'
  import { useAgentsNavigation, useAgentsStore } from '~/features/agent'
  import { useChatsStore } from '~/features/chats'
  import { BaseContainer } from '~/shared/ui/base'
  definePageMeta({
    title: 'Chats',
  })
  const { listChats } = useChatsStore()
  const { listAgents } = useAgentsStore()
  await Promise.all([(listChats(), listAgents())])

  const router = useRouter()

  const currentChatId = computed(() => {
    return router.currentRoute.value.query.id ? Number(router.currentRoute.value.query.id) : undefined
  })

  const { selectedAgent } = useAgentsNavigation()

  const SidebarComponent = computed(() => {
    if (selectedAgent.value) {
      return AgentFullItem
    }
    if (currentChatId.value) {
      return ChatSettings
    }
    return null
  })
</script>

<template>
  <BaseContainer>
    <template #main>
      <div class="chats-base">
        <ChatsHistory />
        <CurrentChat :key="currentChatId" />
      </div>
    </template>
    <template
      v-if="SidebarComponent"
      #additional
    >
      <div class="chats-additional">
        <component :is="SidebarComponent" />
      </div>
    </template>
  </BaseContainer>
</template>

<style lang="scss" scoped>
  .chats-base {
    flex: 1;

    @include flex(row, flex-start, stretch);
  }

  .chats-additional {
    height: 100%;
    background-color: var(--surface-2);
  }
</style>
