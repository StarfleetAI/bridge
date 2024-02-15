<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import utc from 'dayjs/plugin/utc'
  import hljs from 'highlight.js'
  import CopyButtonPlugin from 'highlightjs-copy'
  import { useAgentsStore } from '~/features/agents/store'
  import { useChatsStore, useMessagesStore } from '~/features/chats'
  import { Role, Status } from '~/entities/chat'
  import ChatInput from './ChatInput.vue'
  import ChatMessage from './ChatMessage.vue'

  const copyButtonPlugin = new CopyButtonPlugin()
  hljs.addPlugin(copyButtonPlugin)
  const route = useRoute('chats-id')

  const { createMessage, listMessages, $reset: resetMessagesStore, denyToolCall } = useMessagesStore()

  const chatId = computed(() => (route.params.id ? Number(route.params.id) : undefined))
  if (chatId.value) {
    await listMessages(chatId.value)
  }
  const { messages } = storeToRefs(useMessagesStore())
  const { getById } = useChatsStore()
  const { agents } = storeToRefs(useAgentsStore())

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
    if (chatId.value && messages.value[chatId.value].at(-1)?.status === Status.WAITING_FOR_TOOL_CALL) {
      await denyToolCall(chatId.value)
    }
    createMessage(chatInput.value, chatId.value)
    chatInput.value = ''
  }

  onMounted(async () => {
    await nextTick()
    scrollMessagesListToBottom()
  })

  onBeforeUnmount(() => {
    hljs.removePlugin(copyButtonPlugin)
  })

  const currentChatMessages = computed(() => {
    if (!chatId.value) {
      return []
    }
    return messages.value[chatId.value]
  })
  const currentChat = computed(() => {
    if (!chatId.value) {
      return null
    }
    return getById(chatId.value)
  })

  onBeforeRouteLeave(async () => {
    await resetMessagesStore()
  })
  const chatTitle = computed(() => {
    if (!currentChat.value) {
      return 'New chat'
    }
    if (currentChat.value.title) {
      return currentChat.value.title
    }
    return `Chat #${currentChat.value?.id}`
  })
  const isProcessing = computed(() => {
    if (chatId.value && messages.value[chatId.value]) {
      return messages.value[chatId.value].at(-1)?.status === Status.WRITING
    }
    return false
  })
  const dayjs = useDayjs()
  dayjs.extend(utc)

  const greeting = computed(() => {
    return {
      id: 0,
      chat_id: 0,
      agent_id: agents.value[0].id,
      status: Status.COMPLETED,
      role: Role.SYSTEM,
      content: agents.value[0].system_message,
      prompt_tokens: null,
      completion_tokens: null,
      tool_call_id: null,
      tool_calls: null,
      created_at: dayjs().utc().format('YYYY-MM-DD HH:mm:ss')
    }
  })
</script>

<template>
  <div class="current-chat">
    <div class="current-chat__title">
      {{ chatTitle }}
    </div>
    <div
      ref="messagesListRef"
      class="current-chat__messages-wrapper"
    >
      <div class="current-chat__messages">
        <template v-if="currentChatMessages.length">
          <ChatMessage
            v-for="message in currentChatMessages"
            :key="message.id"
            class="message"
            :message="message"
          />
        </template>
        <template v-else>
          <ChatMessage
            class="message"
            :message="greeting"
          />
        </template>
      </div>
    </div>
    <ChatInput
      v-model="chatInput"
      :is-processing="isProcessing"
      @submit="handleSendMessage"
    />
  </div>
</template>

<style lang="scss" scoped>
  .current-chat {
    position: relative;
    flex: 1;

    @include flex(column, flex-start, stretch);
  }

  .current-chat__title {
    position: absolute;
    top: 0;
    z-index: 2;
    width: 100%;
    height: 56px;
    padding: 18px 48px;
    background-color: var(--surface-1);

    @include font-inter-700(14px, 20px, var(--text-secondary));

    // background-color: rgba(var(--surface-1), 0.2);

    // backdrop-filter: blur(1px);
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

  .message:first-child {
    margin-top: 56px;
  }
</style>
