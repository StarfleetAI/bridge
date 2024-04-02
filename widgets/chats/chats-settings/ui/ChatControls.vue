<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { deleteChat, useChatsNavigation } from '~/features/chats'
  import type { Chat } from '~/entities/chat'

  import { BaseDropdown, BaseDropdownItem } from '~/shared/ui/dropdown'
  import { KebabIcon, CrossIcon, DeleteIcon } from '~/shared/ui/icons'

  const props = defineProps<{ chat: Nullable<Chat> }>()

  const handleDelete = () => {
    deleteChat(props.chat!.id)
    navigateTo('/chats')
  }

  const { setIsSettingsOpened } = useChatsNavigation()
</script>

<template>
  <div class="chat-controls">
    <BaseDropdown v-if="chat">
      <KebabIcon
        height="20"
        width="20"
      />
      <template #content>
        <BaseDropdownItem
          v-close-popper
          class="chat-controls__delete"
          @click="handleDelete"
        >
          <template #icon>
            <DeleteIcon />
          </template>
          <template #label>
            <div class="chat-controls__dellabel">Delete chat</div>
          </template>
        </BaseDropdownItem>
      </template>
    </BaseDropdown>
    <CrossIcon
      width="20"
      height="20"
      @click="setIsSettingsOpened(false)"
    />
  </div>
</template>

<style lang="scss" scoped>
  .chat-controls {
    color: var(--text-tertiary);

    @include flex($gap: 16px, $align: center);
  }

  .chat-controls__delete {
    &:hover {
      background-color: var(--surface-4);
    }

    @include font-inter-500(16px, 22px, var(--status-failed));
    @include flex(row, flex-start, center, 8px);
  }
</style>
