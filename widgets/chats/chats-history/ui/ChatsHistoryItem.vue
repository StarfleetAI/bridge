<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { updateChatTitle, useChatsStore } from '~/features/chats'
  import type { Chat } from '~/entities/chat'
  import { BRIDGE_AGENT_ID } from '~/shared/lib'
  import { BaseDropdown, BaseDropdownItem } from '~/shared/ui/dropdown'
  import {
    BridgeSmallIcon,
    DeleteIcon,
    PenIcon as PenIconAsync,
    KebabIcon,
    NoAvatarIcon,
    PinIcon,
    UnpinIcon,
  } from '~/shared/ui/icons'

  const PenIcon = defineAsyncComponent(PenIconAsync)

  const props = defineProps<{
    chat: Chat
    currentChatId?: number
  }>()
  const { listChats } = useChatsStore()

  const chatToEditTitle = ref<Nullable<number>>(null)

  const setChatToEditTitle = (id: number) => {
    titleToEdit.value = props.chat.title || `Chat #${id}`
  }

  const titleToEdit = ref('')
  const inputRef = ref<Nullable<HTMLInputElement>>(null)
  const handleFocus = () => {
    inputRef.value?.focus()
  }

  const handleClick = (newId: number) => {
    if (props.currentChatId !== newId) {
      chatToEditTitle.value = null
      navigateTo({ name: 'chats', query: { id: newId } })
    } else {
      chatToEditTitle.value = newId
      setChatToEditTitle(newId)
      nextTick(() => {
        handleFocus()
      })
    }
  }

  const handleSaveTitle = async () => {
    if (chatToEditTitle.value) {
      await updateChatTitle({ id: chatToEditTitle.value, title: titleToEdit.value })
      await listChats()
      chatToEditTitle.value = null
      titleToEdit.value = ''
    }
  }
  const handleClickEnter = () => {
    handleSaveTitle()
    inputRef.value?.blur()
  }
  const handleCancelEdit = () => {
    chatToEditTitle.value = null
    inputRef.value?.blur()
  }
  const getChatTitle = (chat: Chat) => {
    if (chat.title) {
      return chat.title
    }
    return `Chat #${chat.id}`
  }

  const { deleteChat, toggleIsPinned } = useChatsStore()

  const pinIcon = computed(() => (props.chat.is_pinned ? UnpinIcon : PinIcon))
  const pinLabel = computed(() => (props.chat.is_pinned ? 'Unpin' : 'Pin'))

  const enableEditing = async () => {
    chatToEditTitle.value = props.chat.id
    setChatToEditTitle(props.chat.id)
    await nextTick()
    handleFocus()
  }
</script>

<template>
  <div
    :key="chat.id"
    :class="['history-item', { active: currentChatId === chat.id }]"
    @click="handleClick(chat.id)"
  >
    <BridgeSmallIcon
      v-if="chat.agents_ids[0] === BRIDGE_AGENT_ID"
      class="history-item__avatar"
    />
    <NoAvatarIcon
      v-else
      class="history-item__avatar"
      width="24px"
      height="24px"
    />
    <input
      ref="inputRef"
      v-model="titleToEdit"
      :class="['history-item__name', { disabled: !chatToEditTitle }]"
      :placeholder="getChatTitle(chat)"
      @keydown.enter="handleClickEnter"
      @keydown.esc="handleCancelEdit"
      @blur="handleSaveTitle"
    />

    <BaseDropdown v-if="!chatToEditTitle">
      <KebabIcon
        height="16px"
        width="16px"
        class="history-item__kebab"
        @click.stop
      />
      <template #content>
        <BaseDropdownItem
          v-close-popper
          @click="enableEditing"
        >
          <template #icon>
            <PenIcon
              width="20px"
              height="20px"
            />
          </template>
          <template #label>Rename</template>
        </BaseDropdownItem>
        <BaseDropdownItem
          v-close-popper
          @click="toggleIsPinned(chat.id)"
        >
          <template #icon>
            <component
              :is="pinIcon"
              width="20px"
              height="20px"
              class="pin-icon"
            />
          </template>
          <template #label>{{ pinLabel }} Chat</template>
        </BaseDropdownItem>
        <BaseDropdownItem
          v-close-popper
          style="color: var(--status-failed)"
          @click="deleteChat(chat.id)"
        >
          <template #icon>
            <DeleteIcon
              width="20px"
              height="20px"
            />
          </template>
          <template #label>Delete</template>
        </BaseDropdownItem>
      </template>
    </BaseDropdown>
  </div>
</template>

<style lang="scss" scoped>
  .history-item {
    display: flex;
    flex: 1;
    overflow: hidden;
    padding: 6px 8px;
    border-radius: 4px;

    &:hover,
    &.active {
      background-color: var(--surface-4);

      .history-item__name {
        background-color: var(--surface-4);

        &::placeholder {
          color: var(--text-primary);
        }
      }
    }

    &:hover {
      .history-item__kebab {
        opacity: 1;
      }
    }

    @include flex(row, flex-start, center, 8px);
  }

  .history-item__avatar {
    flex-shrink: 0;
  }

  .history-item__name {
    display: flex;
    overflow: hidden;
    width: 100%;
    padding: 2px;
    background-color: transparent;
    text-overflow: ellipsis;
    white-space: nowrap;

    &.disabled {
      cursor: pointer;
      pointer-events: none;
      user-select: none;

      &::placeholder {
        color: var(--text-secondary);
      }
    }

    @include font-inter-400(12px, 17px, var(--text-primary));
  }

  .history-item__kebab {
    opacity: 0;
  }

  .pin-icon {
    :deep(path) {
      stroke-width: 1px;
    }
  }
</style>
