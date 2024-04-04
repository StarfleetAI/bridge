<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import utc from 'dayjs/plugin/utc'
  import hljs from 'highlight.js'
  import CopyButtonPlugin from 'highlightjs-copy'
  import { useAgentsStore } from '~/features/agent/store'
  import { useChatsStore, useMessagesStore } from '~/features/chats'
  import { useTasksStore } from '~/features/task'
  import type { Agent } from '~/entities/agents'
  import { Status } from '~/entities/chat'
  import { BRIDGE_AGENT_ID } from '~/shared/lib'

  import { ChatInput } from '~/shared/ui/base'
  import LogItem from './LogItem.vue'

  const copyButtonPlugin = new CopyButtonPlugin()
  hljs.addPlugin(copyButtonPlugin)

  const { listMessages, $reset: resetMessagesStore } = useMessagesStore()
  const { selectedTask: task } = storeToRefs(useTasksStore())

  const chatId = computed(() => task.value?.execution_chat_id)
  if (chatId.value) {
    await listMessages(chatId.value)
  }

  watch(chatId, async (newVal) => {
    if (newVal) {
      await listMessages(newVal)
      await nextTick()
      scrollMessagesListToBottom()
    }
  })
  const { createMessage } = useMessagesStore()
  const { messages } = storeToRefs(useMessagesStore())
  const { getById: getChatById } = useChatsStore()
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
    async () => {
      if (messagesListRef.value && !isScrolling.value && arrivedState.bottom) {
        await nextTick() // await for new message to be rendered
        await nextTick() // await for new message to be highlighted
        scrollMessagesListToBottom()
      }
    },
    { immediate: true, deep: true },
  )

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
    return messages.value[chatId.value]?.filter((message) => message.is_internal_tool_output === false) || []
  })
  const currentChat = computed(() => {
    if (!chatId.value) {
      return null
    }
    return getChatById(chatId.value)
  })

  onBeforeRouteLeave(async () => {
    await resetMessagesStore()
  })
  // const isProcessing = computed(() => {
  //   if (chatId.value && messages.value[chatId.value]) {
  //     return messages.value[chatId.value].at(-1)?.status === Status.WRITING
  //   }
  //   return false
  // })
  const dayjs = useDayjs()
  dayjs.extend(utc)
  const bridgeAgent = computed(() => agents.value.find((agent) => agent.id === BRIDGE_AGENT_ID)!)
  const currentAgent = ref<Agent>(structuredClone(toRaw(bridgeAgent.value)))
  if (currentChat.value?.agents_ids?.length === 1) {
    const agent = getAgentById(currentChat.value?.agents_ids[0])
    if (agent) {
      currentAgent.value = structuredClone(toRaw(agent))
    }
  }

  const logsInput = ref('')
  const isProcessing = computed(() => {
    if (chatId.value && messages.value[chatId.value]) {
      return messages.value[chatId.value].at(-1)?.status === Status.WRITING
    }
    return false
  })

  const handleSendMessage = async () => {
    if (!logsInput.value) {
      return
    }
    createMessage({
      text: logsInput.value,
      agent_id: currentAgent.value.id,
      chat_id: chatId.value,
      model_full_name: currentChat.value?.model_full_name || null,
    })
    logsInput.value = ''
    nextTick(() => {
      scrollMessagesListToBottom()
    })
  }
</script>

<template>
  <div class="task-logs">
    <div
      ref="messagesListRef"
      class="task-logs__messages-wrapper"
    >
      <div class="task-logs__messages">
        <template v-if="currentChatMessages?.length">
          <LogItem
            v-for="message in currentChatMessages"
            :key="message.id"
            class="message"
            :message="message"
          />
        </template>
        <div
          v-else
          class="tasks-logs__empty"
        >
          No logs yet...
        </div>
      </div>
    </div>
    <ChatInput
      v-model="logsInput"
      :is-processing="isProcessing"
      :with-files="false"
      class="task-logs__input"
      @submit="handleSendMessage"
    />
  </div>
</template>

<style lang="scss" scoped>
  .task-logs {
    overflow: hidden;
    width: 100%;
    height: 100%;
    padding-bottom: 24px;

    @include flex(column, flex-start);
  }

  .task-logs__messages-wrapper {
    flex: 1;
    overflow: auto;
    width: 100%;
    height: 100%;
    max-height: calc(100% - 73px);
    padding: 0 24px;
    padding-bottom: 48px;
    transition: all 0.2s ease;

    @include add-scrollbar;
  }

  .task-logs__messages {
    width: 100%;
    padding: 16px 0 0;

    @include flex(column, space-between, stretch, 64px);
  }

  .tasks-logs__empty {
    @include font-inter-400(14px, 20px, var(--text-tertiary));
  }

  .task-logs__input {
    flex-shrink: 0;
    max-height: 82px;
    margin-top: auto;
    padding: 0 24px;
    padding-top: 24px;
    border-top: 0.5px solid var(--text-tertiary);
  }
</style>
