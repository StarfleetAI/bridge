<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useQuery } from '@tanstack/vue-query'
  import { ChatsHistory } from '~/widgets/chats/chats-history'
  import { CurrentChat } from '~/widgets/chats/current-chat'
  import { agentsInjectionKey, listAgents } from '~/features/chats/list-agents'
  import { listChats, type ChatsGroups } from '~/features/chats/list-chats'
  import type { Agent } from '~/entities/agent'
  import { BaseContainer } from '~/shared/ui/base-container'

  definePageMeta({
    title: 'Chats'
  })

  const { data: agents } = useQuery({
    queryKey: ['list-agents'],
    queryFn: listAgents,
    placeholderData: () => []
  })

  provide(agentsInjectionKey, readonly(agents as Ref<Agent[]>))
  const chats = ref<ChatsGroups>([])

  const fetchChats = async () => {
    chats.value = await listChats()
  }

  await fetchChats()
  const currentChatIsEmpty = ref(false)

  const setCurrentChatIsEmpty = (newVal: boolean) => {
    currentChatIsEmpty.value = newVal
  }
</script>

<template>
  <BaseContainer>
    <template #main>
      <div class="chats-base">
        <ChatsHistory
          :chats="chats"
          :current-chat-is-empty="currentChatIsEmpty"
          @update-chats="fetchChats"
        />
        <CurrentChat
          :agents="agents"
          @set-current-chat-is-empty="setCurrentChatIsEmpty"
        />
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
