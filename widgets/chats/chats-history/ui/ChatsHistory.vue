<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useRouteQuery } from '@vueuse/router'
  import { type ChatsGroups, chatsToGroupsByDate, useChatsStore } from '~/features/chats'
  import { updateChatTitle } from '~/features/chats'
  import type { Chat } from '~/entities/chat'
  import { BridgeSmallIcon } from '~/shared/ui/icons'
  import NewChatButton from './NewChatButton.vue'

  const { chats, pinnedChats } = storeToRefs(useChatsStore())
  const { listChats, getById } = useChatsStore()
  const chatsGroups = computed<ChatsGroups>(() => {
    if (!chats.value) {
      return [] as ChatsGroups
    }

    return chatsToGroupsByDate(chats.value)
  })

  const route = useRoute('chats')

  const currentChatId = computed(() => Number(route.query.id))

  const chatToEditTitle = ref<Nullable<number>>(null)

  const setChatToEditTitle = (id: number) => {
    titleToEdit.value = getById(id)?.title || `Chat #${id}`
  }

  const titleToEdit = ref('')
  const inputRef = ref<[HTMLInputElement] | null>(null)
  const handleFocus = () => {
    if (inputRef.value) {
      inputRef.value[0].focus()
    }
  }

  const chatId = useRouteQuery('id', '', {
    transform: (value: string) => (isNaN(Number(value)) ? null : Number(value)),
  })

  const handleClick = (newId: number) => {
    if (currentChatId.value !== newId) {
      chatId.value = newId
      chatToEditTitle.value = null
    } else {
      chatToEditTitle.value = newId
      setChatToEditTitle(newId)
      nextTick(() => {
        handleFocus()
      })
    }
  }
  const handleInput = (event: Event) => {
    titleToEdit.value = (event.target as HTMLInputElement).value
  }

  const getItemComponent = (id: number) => {
    if (id === chatToEditTitle.value) {
      return 'input'
    }
    return 'div'
  }
  const handleSaveTitle = async () => {
    if (chatToEditTitle.value) {
      await updateChatTitle({ id: chatToEditTitle.value, title: titleToEdit.value })
      await listChats()
      chatToEditTitle.value = null
      titleToEdit.value = ''
    }
  }
  const handleCancelEdit = () => {
    chatToEditTitle.value = null
  }
  const getChatTitle = (chat: Chat) => {
    if (chat.title) {
      return chat.title
    }
    return `Chat #${chat.id}`
  }
</script>

<template>
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
        <div
          v-for="chat in pinnedChats"
          :key="chat.id"
          :class="['history-item', { active: currentChatId === chat.id }]"
          @click="handleClick(chat.id)"
        >
          <BridgeSmallIcon />
          <component
            :is="getItemComponent(chat.id)"
            :ref="getItemComponent(chat.id) === 'input' ? 'inputRef' : null"
            :class="['history-item__name', { 'is-input': getItemComponent(chat.id) === 'input' }]"
            :value="titleToEdit"
            @keydown.enter="handleSaveTitle"
            @input="handleInput"
            @keydown.esc="handleCancelEdit"
          >
            {{ getChatTitle(chat) }}
          </component>
        </div>
      </div>
      <div
        v-for="[date, group] in chatsGroups"
        :key="date"
        class="history-group"
      >
        <div class="history-group__title">{{ date }}</div>
        <div
          v-for="chat in group"
          :key="chat.id"
          :class="['history-item', { active: currentChatId === chat.id }]"
          @click="handleClick(chat.id)"
        >
          <BridgeSmallIcon />
          <component
            :is="getItemComponent(chat.id)"
            :ref="getItemComponent(chat.id) === 'input' ? 'inputRef' : null"
            :class="['history-item__name', { 'is-input': getItemComponent(chat.id) === 'input' }]"
            :value="titleToEdit"
            @keydown.enter="handleSaveTitle"
            @input="handleInput"
            @keydown.esc="handleCancelEdit"
          >
            {{ getChatTitle(chat) }}
          </component>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .chats-history {
    width: 200px;
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

  .history-item {
    flex: 1;
    overflow: hidden;
    padding: 6px 8px;
    border-radius: 4px;

    &:hover,
    &.active {
      background-color: var(--surface-4);
      color: var(--text-primary);

      .history-item__name {
        background-color: var(--surface-4);
        color: var(--text-primary);
      }
    }

    @include flex(row, flex-start, center, 8px);
  }

  .history-item__name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;

    &.is-input {
      display: flex;
      width: 100%;
      outline: none;
    }

    @include font-inter-400(12px, 17px, var(--text-secondary));
  }
</style>
