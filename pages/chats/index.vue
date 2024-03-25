<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { AgentFullItem } from '~/widgets/agentFullItem'
  import { ChatsHistory } from '~/widgets/chats/chats-history'
  import { ChatSettings } from '~/widgets/chats/chats-settings'
  import { CurrentChat } from '~/widgets/chats/current-chat'
  import { useAgentsNavigation, useAgentsStore } from '~/features/agent'
  import { useChatsNavigation, useChatsStore } from '~/features/chats'
  import { type ChatSettings as ChatSettingsType } from '~/entities/chat'
  import { BaseContainer } from '~/shared/ui/base'

  const AgentFullItemComponent = defineAsyncComponent(AgentFullItem)
  const ChatSettingsComponent = defineAsyncComponent(ChatSettings)
  definePageMeta({
    title: 'Chats',
  })
  const { listChats, getById: getChatById, updateChatModelFullName } = useChatsStore()
  const { listAgents } = useAgentsStore()
  await Promise.all([(listChats(), listAgents())])

  const router = useRouter()

  const currentChatId = computed(() => {
    return router.currentRoute.value.query.id ? Number(router.currentRoute.value.query.id) : undefined
  })

  const { selectedAgent } = useAgentsNavigation()
  const { chatSettingsIsOpened } = useChatsNavigation()
  const SidebarComponent = computed(() => {
    if (selectedAgent.value) {
      return AgentFullItemComponent
    }
    if (chatSettingsIsOpened.value) {
      return ChatSettingsComponent
    }
    return null
  })

  const chat = computed(() => getChatById(currentChatId.value))
  const currentChatSettings = ref<ChatSettingsType>({ model_full_name: null })
  const updateCurrentChatSettings = (newVal: ChatSettingsType) => {
    currentChatSettings.value = newVal
    if (chat.value) {
      updateChatModelFullName(chat.value.id, newVal.model_full_name)
    }
  }

  watch(
    () => chat.value,
    (newVal) => {
      if (newVal) {
        currentChatSettings.value.model_full_name = newVal.model_full_name
      } else {
        currentChatSettings.value.model_full_name = null
      }
    },
    {
      deep: true,
      immediate: true,
    },
  )
</script>

<template>
  <BaseContainer>
    <template #main>
      <div class="chats-base">
        <ChatsHistory />
        <CurrentChat :settings="currentChatSettings" />
      </div>
    </template>
    <template
      v-if="SidebarComponent"
      #additional
    >
      <div class="chats-additional">
        <component
          :is="SidebarComponent"
          :settings="currentChatSettings"
          @update-settings="updateCurrentChatSettings($event as ChatSettingsType)"
        />
      </div>
    </template>
  </BaseContainer>
</template>

<style lang="scss" scoped>
  .chats-base {
    flex: 1;
    width: 100%;
    min-width: 0;

    @include flex(row, flex-start, stretch);
  }

  .chats-additional {
    width: 100%;
    height: 100%;
    background-color: var(--surface-2);
  }
</style>
