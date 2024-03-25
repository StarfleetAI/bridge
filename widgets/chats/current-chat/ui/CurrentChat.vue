<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import utc from 'dayjs/plugin/utc'
  import hljs from 'highlight.js'
  import CopyButtonPlugin from 'highlightjs-copy'
  import { useAgentsNavigation } from '~/features/agent'
  import { useAgentsStore } from '~/features/agent/store'
  import { useChatsStore, useMessagesStore } from '~/features/chats'
  import type { Agent } from '~/entities/agents'
  import { Status, type ChatSettings } from '~/entities/chat'
  import { BRIDGE_AGENT_ID } from '~/shared/lib'
  import { ChatInput } from '~/shared/ui/base'
  import ChatGreeting from './ChatGreeting.vue'
  import ChatHeader from './ChatHeader.vue'
  import ChatMessage from './ChatMessage.vue'
  import ChatStartPresets from './ChatStartPresets.vue'

  const props = defineProps<{
    settings: ChatSettings
  }>()
  const copyButtonPlugin = new CopyButtonPlugin()
  hljs.addPlugin(copyButtonPlugin)
  const route = useRoute('chats')

  const { createMessage, listMessages, $reset: resetMessagesStore } = useMessagesStore()

  const chatId = computed(() => (route.query.id ? Number(route.query.id) : undefined))
  if (chatId.value) {
    await listMessages(chatId.value)
  }
  const isLoading = ref(false)
  const getMessages = async (id: number) => {
    isLoading.value = true
    await listMessages(id)
    isLoading.value = false
  }
  watch(chatId, async (newVal) => {
    if (newVal) {
      setCurrentChatAgent()
      await getMessages(newVal)
      await nextTick()
      scrollMessagesListToBottom()
    }
  })

  const { messages } = storeToRefs(useMessagesStore())
  const { getById } = useChatsStore()
  const { agents } = storeToRefs(useAgentsStore())
  const { getById: getAgentById } = useAgentsStore()
  const messagesListRef = ref<HTMLDivElement>()

  const scrollMessagesListToBottom = () => {
    if (messagesListRef.value) {
      messagesListRef.value.scrollTo(0, messagesListRef.value.scrollHeight)
    }
  }
  const { isScrolling, arrivedState } = useScroll(messagesListRef, {
    offset: {
      bottom: 140,
    },
  })

  watch(
    () => messages.value,
    () => {
      if (messagesListRef.value && !isScrolling.value && arrivedState.bottom) {
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
    createMessage({
      text: chatInput.value,
      agent_id: currentAgent.value.id,
      chat_id: chatId.value,
      model_full_name: props.settings.model_full_name,
    })
    chatInput.value = ''
    nextTick(() => {
      scrollMessagesListToBottom()
    })
  }
  const selectPreset = (preset: string) => {
    createMessage({
      text: preset,
      agent_id: currentAgent.value.id,
      chat_id: chatId.value,
      model_full_name: props.settings.model_full_name,
    })
  }

  onMounted(async () => {
    await nextTick()
    await nextTick()
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
    return messages.value[chatId.value]?.filter((message) => message.is_internal_tool_output === false)
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
  const setCurrentChatAgent = () => {
    if (currentChat.value?.agents_ids?.length === 1) {
      const agent = getAgentById(currentChat.value?.agents_ids[0])
      if (agent) {
        currentAgent.value = structuredClone(toRaw(agent))
      }
    }
  }

  const handleAgentChange = (agentId: number) => {
    const newAgent = getAgentById(agentId)
    if (newAgent) {
      currentAgent.value = structuredClone(toRaw(newAgent))
    } else {
      currentAgent.value = structuredClone(toRaw(bridgeAgent.value))
    }
  }

  // Handle navigation from agent card
  const { selectedAgent } = useAgentsNavigation()
  if (selectedAgent.value) {
    handleAgentChange(selectedAgent.value)
  }

  const showGreeting = computed(() => {
    return isLoading.value === false && currentChatMessages.value?.length === 0
  })
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
    >
      <div :class="['current-chat__messages', { 'is-greeting': currentChatMessages?.length === 0 }]">
        <template v-if="currentChatMessages?.length">
          <ChatMessage
            v-for="message in currentChatMessages"
            :key="message.id"
            class="message"
            :message="message"
            :is-last="message.id === currentChatMessages.at(-1)?.id"
          />
        </template>
        <template v-if="showGreeting">
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
      class="current-chat__input"
      @submit="handleSendMessage"
    />
  </div>
</template>

<style lang="scss" scoped>
  .current-chat {
    position: relative;
    flex: 1;
    width: 100%;
    min-width: 583px;

    @include flex(column, flex-start, stretch);
  }

  .current-chat__messages-wrapper {
    overflow: hidden auto;
    width: 100%;
    height: 100%;
    padding: 0 24px;
    padding-bottom: 48px;
    transition: all 0.2s ease;

    @include add-scrollbar;
  }

  .current-chat__messages {
    width: 100%;
    max-width: 720px;
    margin: 0 auto;
    padding: 16px 0 0;
    padding-bottom: 16px;

    &.is-greeting {
      height: 100%;
    }

    @include flex(column, space-between, stretch, 64px);
  }

  .current-chat__input {
    margin-top: auto;
    margin-bottom: 32px;
    padding: 0 24px;
  }
</style>
