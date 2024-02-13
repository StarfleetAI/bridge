<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import 'highlight.js/styles/atom-one-dark.min.css'
  import hljs from 'highlight.js'
  import { Marked } from 'marked'
  import { markedHighlight } from 'marked-highlight'
  import { agentsInjectionKey } from '~/features/chats/list-agents'
  import { type Message, Role } from '~/entities/chat'
  import { SystemIcon, NoAvatarIcon } from '~/shared/ui/icons'
  import ToolCall from './ToolCall.vue'

  const props = defineProps<{
    message: Message
  }>()

  const agents = inject(agentsInjectionKey)
  const marked = new Marked(
    markedHighlight({
      langPrefix: 'language-',
      highlight(code, lang) {
        const language = hljs.getLanguage(lang) ? lang : 'plaintext'
        if (['react', 'vue'].includes(lang)) {
          return hljs.highlight(code, { language: 'html' }).value
        }

        return hljs.highlight(code, { language }).value
      }
    })
  )
  const markdown = (text: string) => {
    return marked.parse(text)
  }

  const getAgentById = (id: number) => {
    return agents?.value.find((agent) => agent.id === id)
  }

  const getAuthorName = (message: Message) => {
    switch (message.role) {
      case 'System':
        return 'System'
      case 'User':
        return 'You'
      case 'Assistant':
        if (message.agent_id === null) {
          return 'Unknown Agent'
        }
        return getAgentById(message.agent_id)?.name || 'Unknown Agent'
      case 'Tool':
        return 'Tool'
      default:
        return 'Unknown'
    }
  }
  const getAuthorAvatar = (message: Message) => {
    if (message.role === 'System') {
      return SystemIcon
    }
    return NoAvatarIcon
  }
  const messageAuthor = computed(() => {
    return {
      name: getAuthorName(props.message),
      avatar: getAuthorAvatar(props.message)
    }
  })
  const dayjs = useDayjs()
  const createdAt = computed(() => dayjs(props.message.created_at).format('MMM D, YYYY, hh:mm'))
</script>

<template>
  <div class="message">
    <component
      :is="messageAuthor.avatar"
      class="author__avatar"
    />
    <div class="message__body">
      <div class="message__body-top">
        <div class="message__author">{{ messageAuthor.name }}</div>
        <div class="message__timestamp">{{ createdAt }}</div>
      </div>
      <div
        :class="[
          'message__content',
          {
            system: message.role === Role.SYSTEM,
            assistant: message.role === Role.ASSISTANT,
            tool: message.role === Role.TOOL
          }
        ]"
      >
        <div
          v-if="message.content?.length > 0 && message.role !== Role.TOOL"
          class="message__content-markdown"
          v-html="markdown(message.content)"
        />
        <div v-if="message.content?.length > 0 && message.role === Role.TOOL">
          <pre><code>{{ message.content }}</code></pre>
        </div>
        <ToolCall
          v-if="message.tool_calls"
          :message="message"
        />
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .message {
    gap: 8px;

    @include flex(row, flex-start, stretch);
  }

  .author__avatar {
    flex-shrink: 0;
    color: var(--text-tertiary);
  }

  .message__body {
    flex: 1 0;
    gap: 8px;

    @include flex(column, flex-start, stretch);
  }

  .message__body-top {
    width: 100%;
    height: 24px;

    @include flex(row, space-between, center);
  }

  .message__author {
    @include font-inter-700(14px, 20px, var(--text-tertiary));
  }

  .message__timestamp {
    @include font-inter-400(12px, 17px, var(--text-tertiary));
  }

  .message__content {
    padding: 0 12px;
    border-radius: 6px;

    &.system {
      padding: 12px;
      background-color: var(--surface-2);
      color: var(--text-secondary);
      box-shadow: -2px 0 0 0 var(--statuses-paused);
    }

    @include font-inter-400(14px, 20px, var(--text-primary));
  }

  .message__content-markdown {
    gap: 1.25em;

    @include flex(column, flex-start, flex-start);
  }
</style>
