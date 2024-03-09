<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import utc from 'dayjs/plugin/utc'
  import hljs from 'highlight.js'
  import CopyButtonPlugin from 'highlightjs-copy'
  import { useAgentsStore } from '~/features/agent/store'
  import { useChatsStore, useMessagesStore } from '~/features/chats'
  import type { Agent } from '~/entities/agents'
  import { Status } from '~/entities/chat'
  import { BRIDGE_AGENT_ID } from '~/shared/lib'
  import ChatGreeting from './ChatGreeting.vue'
  import ChatHeader from './ChatHeader.vue'
  import ChatInput from './ChatInput.vue'
  import ChatMessage from './ChatMessage.vue'
  import ChatStartPresets from './ChatStartPresets.vue'

  const copyButtonPlugin = new CopyButtonPlugin()
  hljs.addPlugin(copyButtonPlugin)
  const route = useRoute('chats')

  const { createMessage, listMessages, $reset: resetMessagesStore } = useMessagesStore()

  const chatId = computed(() => (route.query.id ? Number(route.query.id) : undefined))
  if (chatId.value) {
    await listMessages(chatId.value)
  }
  const { messages } = storeToRefs(useMessagesStore())
  const { getById } = useChatsStore()
  const { agents } = storeToRefs(useAgentsStore())
  const { getById: getAgentById } = useAgentsStore()
  const messagesListRef = ref<HTMLDivElement>()

  const isAutoScrollEnabled = ref(true)
  const handleScroll = () => {
    const scrollHeight = messagesListRef.value!.scrollHeight
    const scrollTop = messagesListRef.value!.scrollTop
    const clientHeight = messagesListRef.value!.clientHeight
    isAutoScrollEnabled.value = scrollHeight - scrollTop <= clientHeight + 30
  }
  const scrollMessagesListToBottom = () => {
    if (messagesListRef.value) {
      messagesListRef.value.scrollTo(0, messagesListRef.value.scrollHeight)
    }
  }
  watch(
    () => messages.value,
    () => {
      if (messagesListRef.value && isAutoScrollEnabled.value) {
        scrollMessagesListToBottom()
      }
    },
    { immediate: true, deep: true },
  )

  const chatInput = ref('')

  const handleSendMessage = async () => {
    if (!chatInput.value) {
      return
    }
    createMessage(chatInput.value, currentAgent.value.id, chatId.value)
    chatInput.value = ''
  }
  const selectPreset = (preset: string) => {
    createMessage(preset, currentAgent.value.id, chatId.value)
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
  const isProcessing = computed(() => {
    if (chatId.value && messages.value[chatId.value]) {
      return messages.value[chatId.value].at(-1)?.status === Status.WRITING
    }
    return false
  })
  const dayjs = useDayjs()
  dayjs.extend(utc)
  const bridgeAgent = computed(() => agents.value.find((agent) => agent.id === BRIDGE_AGENT_ID)!)
  const currentAgent = ref<Agent>(structuredClone(toRaw(bridgeAgent.value)))
  if (currentChat.value?.agents_ids.length === 1) {
    const agent = getAgentById(currentChat.value?.agents_ids[0])
    if (agent) {
      currentAgent.value = structuredClone(toRaw(agent))
    }
  }

  const handleAgentChange = (agentId: number) => {
    const newAgent = agents.value.find((agent) => agent.id === agentId)
    if (newAgent) {
      currentAgent.value = structuredClone(toRaw(newAgent))
    } else {
      currentAgent.value = structuredClone(toRaw(bridgeAgent.value))
    }
  }

  // const isScrollingTimer = ref<NodeJS.Timer>()
  // const handleShowScroll = (show: boolean) => {
  //   if (show) {
  //     messagesListRef.value?.classList.add('is-scrolling')
  //     if (isScrollingTimer.value) {
  //       clearTimeout(isScrollingTimer.value)
  //     }
  //   } else {
  //     isScrollingTimer.value = setTimeout(() => {
  //       messagesListRef.value?.classList.remove('is-scrolling')
  //     }, 500)
  //   }
  // }
</script>

<template>
  <div class="current-chat">
    <ChatHeader
      :chat="currentChat"
      :agent="currentAgent"
    />
    <div
      ref="messagesListRef"
      class="current-chat__messages-wrapper"
      @scroll="handleScroll"
    >
      <div :class="['current-chat__messages', { 'is-greeting': currentChatMessages?.length === 0 }]">
        <template v-if="currentChatMessages?.length">
          <ChatMessage
            v-for="message in currentChatMessages"
            :key="message.id"
            class="message"
            :message="message"
          />
        </template>
        <template v-else>
          <ChatGreeting
            class="message"
            :agent="currentAgent"
            @change-agent="handleAgentChange"
          />
          <ChatStartPresets @select="selectPreset" />
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

  .current-chat__messages-wrapper {
    flex: 1;
    overflow: hidden;
    overflow: auto;
    width: 100%;
    height: 100%;
    padding: 0 24px;
    transition: all 0.2s ease;

    @include add-scrollbar;
  }

  .current-chat__messages {
    flex: 1;
    width: 100%;
    max-width: 680px;
    margin: 0 auto;
    padding: 16px 0 48px;

    &.is-greeting {
      height: 100%;
    }

    @include flex(column, flex-start, stretch, 32px);
  }
</style>
