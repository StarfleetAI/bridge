<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { ChatsHistory } from '~/widgets/chats/chats-history'
  import { CurrentChat } from '~/widgets/chats/current-chat'
  import { useAgentsStore } from '~/features/agents'
  import { useChatsStore } from '~/features/chats'
  import { BaseContainer } from '~/shared/ui/base'

  definePageMeta({
    title: 'Chats'
  })
  const { listChats } = useChatsStore()
  const { listAgents } = useAgentsStore()
  await Promise.all([(listChats(), listAgents())])
</script>

<template>
  <BaseContainer>
    <template #main>
      <div class="chats-base">
        <ChatsHistory />
        <CurrentChat />
      </div>
    </template>
  </BaseContainer>
</template>

<style lang="scss" scoped>
  .chats-base {
    flex: 1;

    @include flex(row, flex-start, stretch);
  }
</style>
