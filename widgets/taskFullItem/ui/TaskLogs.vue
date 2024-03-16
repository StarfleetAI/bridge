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
  import { BRIDGE_AGENT_ID } from '~/shared/lib'

  import LogItem from './LogItem.vue'

  const copyButtonPlugin = new CopyButtonPlugin()
  hljs.addPlugin(copyButtonPlugin)
  const route = useRoute('chats')

  const { listMessages, $reset: resetMessagesStore } = useMessagesStore()
  const { getById: getTaskById } = useTasksStore()
  const taskId = computed(() => (route.query.task ? Number(route.query.task) : undefined))
  const task = computed(() => {
    return getTaskById(taskId.value!)
  })
  const chatId = computed(() => task.value?.execution_chat_id)
  if (chatId.value) {
    await listMessages(chatId.value)
  }

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
    () => {
      if (messagesListRef.value && !isScrolling.value && arrivedState.bottom) {
        scrollMessagesListToBottom()
      }
    },
    { immediate: true, deep: true },
  )

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
    return messages.value[chatId.value]
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
</script>

<template>
  <div
    ref="messagesListRef"
    class="task-logs__messages-wrapper"
  >
    <div class="task-logs__messages">
      <template v-if="!currentChatMessages?.length">
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
</template>

<style lang="scss" scoped>
  .task-logs__messages-wrapper {
    flex: 1;
    overflow: auto;
    width: 100%;
    height: 100%;
    max-height: 545px;
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
</style>
