<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import 'highlight.js/styles/atom-one-dark.min.css'
  import hljs from 'highlight.js'
  import { useAgentsStore } from '~/features/agents'
  import { type Message, Role, type ToolCall as ToolCallType } from '~/entities/chat'
  import { getMarkdown } from '~/shared/lib'
  import { SystemIcon, NoAvatarIcon } from '~/shared/ui/icons'
  import ToolCall from './ToolCall.vue'

  const props = defineProps<{
    message: Message
  }>()

  const { agents } = storeToRefs(useAgentsStore())

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
      avatar: getAuthorAvatar(props.message),
    }
  })
  const dayjs = useDayjs()
  const createdAt = computed(() => {
    let dateString = props.message.created_at

    if (props.message.created_at.at(-1) !== 'Z') {
      dateString += 'Z'
    }
    return dayjs(dateString).format('MMM D, YYYY, HH:mm')
  })

  const toolCalls = computed<ToolCallType[]>(() => {
    return props.message.tool_calls ? JSON.parse(props.message.tool_calls) : ([] as ToolCallType[])
  })

  const messageRef = ref<HTMLDivElement>()

  watch(
    () => [props.message, messageRef.value],
    () => {
      if (props.message.content) {
        messageRef.value?.querySelectorAll('pre code').forEach((el) => {
          if (el.getAttribute('data-highlighted') !== 'yes') {
            // add data-language attribute to show it in the highlighter
            const lang = el.className
              .split(' ')
              .find((item) => item.startsWith('language-'))
              ?.slice(9)
            if (lang) {
              if (!hljs.getLanguage(lang)) {
                el.classList.value = 'language-html'
              }
              el.parentElement?.setAttribute('data-language', lang)
              hljs.highlightElement(el as HTMLElement)
            }
          }
        })
      }
    },
    {
      deep: true,
      immediate: true,
    },
  )
  const markedContent = computed(() => {
    return getMarkdown(props.message.content)
  })
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
            tool: message.role === Role.TOOL,
          },
        ]"
      >
        <div
          v-if="message.content?.length > 0 && message.role !== Role.TOOL"
          ref="messageRef"
          class="message__content-markdown"
          v-html="markedContent"
        />
        <div
          v-if="message.content?.length > 0 && message.role === Role.TOOL"
          class="tool__content"
          v-html="message.content"
        />
        <div
          v-if="toolCalls.length"
          class="message__toolcalls"
        >
          <ToolCall
            v-for="toolCall in toolCalls"
            :key="toolCall.id"
            :tool-call="toolCall"
            :status="message.status"
            :message-id="message.id"
          />
        </div>
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
    border-radius: 6px;

    &.system {
      padding: 12px;
      background-color: var(--surface-2);
      color: var(--text-secondary);
      box-shadow: -2px 0 0 0 var(--status-paused);
    }

    @include font-inter-400(14px, 20px, var(--text-primary));
  }

  .tool__content {
    white-space: pre-wrap;
    word-break: break-word;

    @include font-mono;
  }

  .message__content-markdown {
    gap: 1.25em;
    cursor: auto;
    user-select: initial;

    @include flex(column, flex-start, flex-start);
  }

  :deep(.hljs-copy-wrapper) {
    position: relative;
    overflow: hidden;
    width: 100%;
    max-width: 646px;
    border-radius: 6px;

    &:before {
      content: attr(data-language);
      order: 1;
      width: 100%;
      padding: 8px 12px;
      background-color: var(--surface-5);
      font-family: Inter, sans-serif;

      @include font-inter-500(14px, 20px, var(--text-primary));
    }

    & > code {
      overflow: visible auto;

      @include add-scrollbar;
    }

    @include flex(column-reverse);
  }

  :deep(.hljs-copy-button) {
    position: absolute;
    top: 8px;
    right: 12px;
    display: flex;
    gap: 4px;
    justify-content: flex-end;
    align-items: center;
    align-self: flex-end;
    width: auto;
    min-width: 52px;
    padding-left: 16px;
    font-family: Inter, sans-serif;
    text-align: end;

    &:before {
      content: '';
      width: 16px;
      height: 16px;
      background: url('~/assets/svg/copy-icon.svg') no-repeat left;
    }

    @include font-inter-500(14px, 20px, var(--text-secondary));
  }

  :deep(.hljs-copy-alert) {
    display: none;
  }

  :deep(code[data-highlighted='yes']) {
    background-color: var(--surface-3);
  }

  :deep(pre code.hljs) {
    padding: 8px 12px;
  }
</style>
