<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<!-- TODO: Make reusable component for all messages -->

<script lang="ts" setup>
  // eslint-disable-next-line boundaries/element-types
  import { ToolCall } from '~/widgets/chats/current-chat'
  import { useAgentsStore } from '~/features/agent'
  import { approveToolCall, denyToolCall } from '~/features/chats'
  import { type Message, Role, type ToolCall as ToolCallType, Status } from '~/entities/chat'
  import { utcToLocalTime, getTimeAgo, highlightCode } from '~/shared/lib'
  import { CopyButton } from '~/shared/ui/base'
  import { SystemIcon, NoAvatarIcon, CheckIcon, CrossIcon, ChevronDownIcon } from '~/shared/ui/icons'

  const props = defineProps<{
    message: Message
  }>()

  const { getById: getAgentById } = useAgentsStore()
  const currentAgent = computed(() => {
    return getAgentById(props.message.agent_id)
  })

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
  const createdAt = computed(() => {
    const localTime = utcToLocalTime(props.message.created_at)
    return getTimeAgo({ date: localTime.toDate(), dateFormat: 'DD.MM.YYYY, HH:mm', fullUnit: true }).value
  })

  const toolCalls = computed<ToolCallType[]>(() => {
    return props.message.tool_calls ? JSON.parse(props.message.tool_calls) : ([] as ToolCallType[])
  })

  const messageRef = ref<HTMLDivElement>()
  const parseAndHighlightContent = () => {
    if (messageRef.value) {
      highlightCode(messageRef.value)
    }
  }
  onMounted(() => {
    parseAndHighlightContent()
  })
  watch(
    () => props.message,
    async () => {
      await nextTick()
      parseAndHighlightContent()
    },
    {
      deep: true,
    },
  )

  const showActions = computed(() => {
    return props.message.status === Status.WAITING_FOR_TOOL_CALL
  })

  const showMore = ref(false)
  const toggleShowMore = () => {
    showMore.value = !showMore.value
  }
  const showMoreButtonIsVisible = computed(() => {
    if (!messageRef.value || isEditing.value) {
      return false
    }
    return props.message.role === Role.SYSTEM && messageRef.value.scrollHeight > messageRef.value.clientHeight
  })
  const showMoreButtonText = computed(() => {
    return showMore.value ? 'Collapse' : 'Expand'
  })

  const isEditing = ref(false)
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
          :class="['message__content-markdown', { system: message.role === Role.SYSTEM, full: showMore }]"
          v-html="message.content"
        />
        <div
          v-if="showMoreButtonIsVisible"
          :class="['show-more', { visible: showMore }]"
          @click="toggleShowMore"
        >
          {{ showMoreButtonText }}
          <ChevronDownIcon />
        </div>

        <div
          v-if="message.content?.length > 0 && message.role === Role.TOOL"
          class="tool__content-wrapper"
        >
          <div class="tool__content-header">
            <CopyButton :content="message.content" />
          </div>
          <div
            class="tool__content"
            v-html="message.content"
          />
        </div>
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
            :current-agent="currentAgent"
          />
          <div
            v-if="showActions"
            class="tool__actions"
          >
            <div
              class="tool__btn approve"
              @click="approveToolCall(Number(message.id))"
            >
              <CheckIcon />
              Approve
            </div>
            <div
              class="tool__btn deny"
              @click="denyToolCall(Number(message.id))"
            >
              <CrossIcon />
              Deny
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .message {
    z-index: 2;
    gap: 8px;
    width: 100%;

    &:hover {
      & .message__timestamp {
        display: block;
      }
    }

    @include flex(row, flex-start, stretch);
  }

  .author__avatar {
    flex-shrink: 0;
    color: var(--text-tertiary);
  }

  .message__body {
    position: relative;
    gap: 8px;
    width: calc(100% - 26px);

    @include flex(column, flex-start, flex-start);
  }

  .message__body-top {
    width: 100%;
    height: 24px;

    @include flex(row, space-between, center);
  }

  .message__author {
    @include font-inter-500(14px, 20px, var(--text-tertiary));
  }

  .message__timestamp {
    display: none;

    @include font-inter-400(12px, 17px, var(--text-tertiary));
  }

  .message__content {
    width: 100%;
    border-radius: 6px;
    word-break: break-word;

    &.system {
      padding: 12px;
      background-color: var(--surface-2);
      color: var(--text-secondary);
      box-shadow: -2px 0 0 0 var(--status-paused);
    }

    @include font-inter-400(16px, 22px, var(--text-primary));
    @include flex(column, $gap: 16px);
  }

  .tool__content-wrapper {
    overflow: hidden;
    border-radius: 6px;
    background-color: var(--surface-2);

    @include flex(column, flex-start, flex-start);
  }

  .tool__content-header {
    width: 100%;
    padding: 8px 12px;
    background-color: var(--surface-5);

    @include flex($justify-content: flex-end);
  }

  .tool__content {
    padding: 8px 12px;
    font-size: 14px;
    white-space: pre-wrap;
    word-break: break-word;
    cursor: auto;
    user-select: initial;

    @include font-mono;
  }

  .tool__actions {
    gap: 16px;
    cursor: default;
    user-select: none;

    @include flex(row);
  }

  .tool__btn {
    gap: 8px;
    width: 50%;
    height: 32px;
    border-radius: 6px;

    &.approve {
      background-color: var(--status-done);
    }

    &.deny {
      background-color: var(--status-failed);
    }

    @include font-inter-500(14px, 20px, var(--text-on-button));
    @include flex(row, center, center);
  }

  .message__toolcalls {
    width: 100%;

    @include flex(column, flex-start, stretch, 16px);
  }

  .message__content-markdown {
    cursor: auto;
    user-select: initial;

    &.system {
      overflow: hidden;
      min-height: 20px;

      &.full {
        height: auto;
      }

      &:not(.full) {
        @include line-clamp(2);
      }
    }

    @include flex(column, flex-start, flex-start, 16px);
  }

  :deep(.hljs-copy-wrapper) {
    position: relative;
    overflow: hidden;
    width: 100%;
    max-width: 100%;
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
      overflow: auto;
      overflow-y: hidden;
      overscroll-behavior: auto;

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
    border: none;
    background-color: transparent;
    font-family: Inter, sans-serif;
    text-align: end;
    cursor: default;

    &:before {
      content: '';
      width: 16px;
      height: 16px;
      background: transparent url('~/assets/svg/copy-icon.svg') no-repeat left;
    }

    @include font-inter-500(14px, 20px, var(--text-secondary));
  }

  .show-more {
    gap: 4px;

    &.visible {
      & svg {
        transform: rotate(180deg);
      }
    }

    @include font-inter-400(12px, 17px, var(--text-tertiary));
    @include flex(row, flex-start, center);
  }

  :deep(.hljs-copy-alert) {
    display: none;
  }

  :deep(code[data-highlighted='yes']) {
    background-color: var(--surface-3);
  }

  :deep(pre code) {
    white-space: pre-wrap;
  }

  :deep(pre code.hljs) {
    padding: 8px 12px;
    white-space: pre;
  }
</style>
