<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { type ChatsGroups, chatsToGroupsByDate, useChatsStore } from '~/features/chats'
  import { ResizableContainer } from '~/shared/ui/base'
  import ChatsHistoryItem from './ChatsHistoryItem.vue'
  import NewChatButton from './NewChatButton.vue'

  const { chats, pinnedChats } = storeToRefs(useChatsStore())
  const chatsGroups = computed<ChatsGroups>(() => {
    if (!chats.value) {
      return [] as ChatsGroups
    }

    return chatsToGroupsByDate(chats.value)
  })

  const route = useRoute('chats')

  const currentChatId = computed(() => Number(route.query.id))
</script>

<template>
  <ResizableContainer
    min-width="172px"
    max-width="320px"
    direction="right"
  >
    <div class="chats-history">
      <div class="history__control">
        Chats
        <NewChatButton />
      </div>

      <div
        ref="chatsListRef"
        class="chats-list"
      >
        <div
          v-if="pinnedChats.length"
          class="history-group"
        >
          <div class="history-group__title">Pinned</div>
          <ChatsHistoryItem
            v-for="chat in pinnedChats"
            :key="chat.id"
            :chat="chat"
            :current-chat-id="currentChatId"
          />
        </div>
        <div
          v-for="[date, group] in chatsGroups"
          :key="date"
          class="history-group"
        >
          <div class="history-group__title">{{ date }}</div>
          <ChatsHistoryItem
            v-for="chat in group"
            :key="chat.id"
            :chat="chat"
            :current-chat-id="currentChatId"
          />
        </div>
      </div>
    </div>
  </ResizableContainer>
</template>

<style lang="scss" scoped>
  .chats-history {
    width: 100%;
    height: 100%;
    background-color: var(--surface-7);
    font-size: 12px;
    line-height: 17px;

    @include flex(column, flex-start, stretch);
  }

  .history__control {
    flex-shrink: 0;
    height: 56px;
    padding: 0 16px;
    color: var(--text-tertiary);

    @include font-inter-700(16px, 22px, var(--text-secondary));
    @include flex(row, space-between, center, 8px);
  }

  .chats-list {
    overflow: auto;
    padding-bottom: 32px;

    @include add-scrollbar;
    @include flex(column, flex-start, stretch);
  }

  .history-group {
    padding: 32px 8px 0;

    @include flex(column);
  }

  .history-group__title {
    padding: 0 8px 8px;

    @include font-inter-400(12px, 17px, var(--text-tertiary));
  }
</style>
