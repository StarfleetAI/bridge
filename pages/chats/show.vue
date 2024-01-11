<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<template>
  <div class="flex flex-col h-5/6">
    <!-- Chat Messages Window -->
    <div class="flex-grow overflow-y-auto p-4">
      <div v-if="chatMessages" class="flex flex-col space-y-4 p-3">
        <div v-for="message in chatMessages" :key="message.id" class="break-words p-2 bg-gray-800 rounded-lg shadow">
          <p>
            <strong>{{ authorName(message) }}</strong>
          </p>

          <div v-if="message.status === Status.WRITING">
            <p>
              <em>is writing...</em>
            </p>
          </div>
          <div v-else>
            <!-- eslint-disable-next-line vue/no-v-html -->
            <div v-if="message.content?.length > 0" v-html="markdown(message.content)" />
            <div v-if="message?.tool_calls">
              <p>
                <strong>Tool Calls</strong>
              </p>
              <ul>
                <pre><code>{{ message.tool_calls }}</code></pre>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Text Input -->
    <div class="border-t-2 border-gray-200 py-3">
      <form @submit.prevent="sendMessage">
        <input
          v-model="newMessage.text"
          type="text"
          placeholder="Type a message..."
          class="w-full p-2 rounded-lg focus:outline-none focus:ring focus:border-blue-300 text-gray-700"
        >
      </form>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { marked } from 'marked'
import type { CreateMessage, Message } from '@/store/messages'
import { useAgentsStore } from '@/store/agents'
import { useChatsStore } from '@/store/chats'
import { Status, useMessagesStore } from '@/store/messages'

const agentsStore = useAgentsStore()
const chatsStore = useChatsStore()
const messagesStore = useMessagesStore()

const newMessage = ref<CreateMessage>({
  chat_id: Number(useRoute().query.id),
  text: ''
})

const chatMessages = computed(() => {
  return messagesStore.listByChatId(Number(useRoute().query.id))
})

const sendMessage = async () => {
  const msg = { ...newMessage.value }
  newMessage.value.text = ''
  await messagesStore.createMessage(msg)
}

const markdown = (text: string) => {
  return marked.parse(text)
}

const authorName = (message: Message) => {
  switch (message.role) {
    case 'System':
      return 'System'
    case 'User':
      return 'You'
    case 'Assistant':
      if (message.agent_id === null) {
        return 'Unknown Agent'
      }

      return agentsStore.getById(message.agent_id)?.name || 'Unknown Agent'
    case 'Tool':
      return 'Tool'
    default:
      return 'Unknown'
  }
}

onMounted(async () => {
  messagesStore.$reset()

  await Promise.all([
    chatsStore.getChat({ id: Number(useRoute().query.id) }),
    messagesStore.listMessages({ chat_id: Number(useRoute().query.id) })
  ])
})

definePageMeta({
  title: 'Chat'
})
</script>

<style>
pre>code {
  margin-top: 1rem;
  margin-bottom: 1rem;
  display: block;
  background-color: #1a202c;
  color: #f7fafc;
  padding: 1rem;
  border-radius: 0.25rem;
}
</style>
