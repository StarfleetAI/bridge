<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useQuery } from '@tanstack/vue-query'
  import type { UnlistenFn } from '@tauri-apps/api/event'
  import { listen } from '@tauri-apps/api/event'
  import { listAbilities, abilitiesInjectionKey } from '~/features/abilities/list-abilities'
  import { createMessage } from '~/features/chats/create-message'
  import { getChat } from '~/features/chats/get-chat'
  import { listChatMessages } from '~/features/chats/list-chat-messages'
  import type { Ability } from '~/entities/ability'
  import type { Message } from '~/entities/chat'
  import ChatInput from './ChatInput.vue'
  import ChatMessage from './ChatMessage.vue'

  const emits = defineEmits<{
    (e: 'set-current-chat-is-empty', val: boolean): void
  }>()
  const route = useRoute('chats-id')
  const isNewChat = computed(() => route.params.id === undefined)

  const { data: chatInfo } = useQuery({
    queryKey: ['get-chat', route.params.id],
    queryFn: () => getChat(Number(route.params.id))
    // enabled: chatFetchEnabled
  })
  const { data: abilities } = useQuery({
    queryKey: ['get-chat-abilities', route.params.id],
    queryFn: listAbilities
  })
  provide(abilitiesInjectionKey, readonly(abilities as Ref<Ability[]>))

  const messages = ref<Message[]>([])
  if (!isNewChat.value) {
    messages.value = await listChatMessages(Number(route.params.id))
    if (messages.value.length === 1 && messages.value[0].role === 'System') {
      emits('set-current-chat-is-empty', true)
    }
  }

  const messagesListRef = ref<HTMLDivElement>()
  const scrollMessagesListToBottom = () => {
    if (messagesListRef.value) {
      messagesListRef.value.scrollTo(0, messagesListRef.value.scrollHeight)
    }
  }
  watch(() => messages.value, scrollMessagesListToBottom, { immediate: true, deep: true })

  const chatInput = ref('')

  const handleSendMessage = async () => {
    if (!chatInput.value) {
      return
    }

    createMessage({ chat_id: Number(route.params.id), text: chatInput.value })
    chatInput.value = ''
  }

  let msgCreatedUnlisten: Promise<UnlistenFn>
  let msgUpdatedUnlisten: Promise<UnlistenFn>

  onMounted(async () => {
    await nextTick()

    scrollMessagesListToBottom()

    msgCreatedUnlisten = listen('messages:created', (event) => {
      messages.value?.push(event.payload as Message)
    })

    msgUpdatedUnlisten = listen('messages:updated', (event) => {
      const msg = event.payload as Message
      const idx = messages.value?.findIndex((m) => m.id === msg.id)

      if (idx) {
        messages.value?.splice(idx, 1, msg)
      }
    })
  })

  onBeforeUnmount(async () => {
    await msgCreatedUnlisten
    await msgUpdatedUnlisten
  })
</script>

<template>
  <div class="current-chat">
    <div class="current-chat__title">
      {{ chatInfo?.title }}
    </div>
    <div class="current-chat__messages-wrapper">
      <div
        ref="messagesListRef"
        class="current-chat__messages"
      >
        <ChatMessage
          v-for="message in messages"
          :key="message.id"
          class="message"
          :message="message"
        >
          {{ message.content }}
        </ChatMessage>
      </div>
    </div>
    <ChatInput
      v-model="chatInput"
      @submit="handleSendMessage"
    />
  </div>
</template>

<style lang="scss" scoped>
  .current-chat {
    flex: 1;

    @include flex(column, flex-start, stretch);
  }

  .current-chat__title {
    height: 56px;
  }

  .current-chat__messages-wrapper {
    flex: 1;
    overflow-y: scroll;
    width: 100%;
    height: calc(100vh - 242px);
    height: calc(100svh - 242px);
    padding: 0 48px;
    padding-bottom: 48px;

    @include add-scrollbar;
  }

  .current-chat__messages {
    flex: 1;
    gap: 32px;
    width: 100%;
    max-width: 680px;
    margin: 0 auto;

    @include flex(column, flex-start, stretch);
  }
</style>
