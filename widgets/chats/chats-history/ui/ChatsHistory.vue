<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { type ChatsGroups, chatsToGroupsByDate, useChatsStore } from '~/features/chats'
  import { updateChatTitle } from '~/features/chats'
  import type { Chat } from '~/entities/chat'
  import NewChatButton from './NewChatButton.vue'

  const { chats } = storeToRefs(useChatsStore())
  const { listChats } = useChatsStore()
  const chatsGroups = computed<ChatsGroups>(() => {
    if (!chats.value) {
      return [] as ChatsGroups
    }

    return chatsToGroupsByDate(chats.value)
  })

  const route = useRoute('chats-id')

  const currentChatId = computed(() => Number(route.params.id))

  const chatToEditTitle = ref<Nullable<number>>(null)

  const setChatToEditTitle = (id: number) => {
    titleToEdit.value = chats.value?.find((chat) => chat.id === id)?.title || `Chat #${id}`
  }

  const titleToEdit = ref('')
  const inputRef = ref<[HTMLInputElement] | null>(null)
  const handleFocus = () => {
    if (inputRef.value) {
      inputRef.value[0].focus()
    }
  }

  const handleClick = (chatId: number) => {
    if (currentChatId.value !== chatId) {
      navigateTo({ name: 'chats-id', params: { id: chatId } })
      chatToEditTitle.value = null
    } else {
      chatToEditTitle.value = chatId
      setChatToEditTitle(chatId)
      nextTick(() => {
        handleFocus()
      })
    }
  }
  const handleInput = (event: Event) => {
    titleToEdit.value = (event.target as HTMLInputElement).value
  }

  const getItemComponent = (chatId: number) => {
    if (chatId === chatToEditTitle.value) {
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
  const getChatTitle = (chat: Chat) => {
    if (chat.title) {
      return chat.title
    }
    return `Chat #${chat.id}`
  }
</script>

<template>
  <div class="chats-history">
    <NewChatButton />
    <div class="chats-list">
      <div
        v-for="[date, group] in chatsGroups"
        :key="date"
        class="history-group"
      >
        <div class="history-group__title">{{ date }}</div>
        <component
          :is="getItemComponent(chat.id)"
          v-for="chat in group"
          :key="chat.id"
          :ref="getItemComponent(chat.id) === 'input' ? 'inputRef' : null"
          :class="[
            'history-item',
            { active: currentChatId === chat.id, 'is-input': getItemComponent(chat.id) === 'input' }
          ]"
          :value="titleToEdit"
          @click="handleClick(chat.id)"
          @keydown.enter="handleSaveTitle"
          @input="handleInput"
        >
          {{ getChatTitle(chat) }}
        </component>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .chats-history {
    width: 200px;
    height: 100%;
    padding: 12px 3px 12px 12px;
    border-right: 1px solid var(--border-3);
    font-size: 12px;
    line-height: 17px;

    @include flex(column, flex-start, stretch);
  }

  .history-group {
    @include flex(column);
  }

  .chats-list {
    gap: 32px;
    overflow-y: auto;

    @include add-scrollbar;
    @include flex(column, flex-start, stretch);
  }

  .history-group__title {
    padding-bottom: 8px;
    padding-left: 8px;
    color: var(--text-tertiary);
  }

  .history-item {
    width: 176px;
    padding: 6px 8px;
    border-radius: 4px;
    color: var(--text-secondary);
    font-weight: 500;

    &:hover,
    &.active {
      background-color: var(--surface-4);
    }

    &.is-input {
      box-sizing: border-box;
      outline: none;
    }
  }
</style>
