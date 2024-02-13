<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useCreateChat } from '~/features/chats/create-chat'
  import { type ChatsGroups } from '~/features/chats/list-chats'
  import type { Chat } from '~/entities/chat'
  import { BRIDGE_AGENT_ID } from '~/shared/lib'
  import NewChatButton from './NewChatButton.vue'

  const props = defineProps<{
    currentChatIsEmpty: boolean
    chats: ChatsGroups
  }>()
  const emits = defineEmits<{
    (e: 'update-chats'): void
  }>()

  const router = useRouter()
  const route = useRoute('chats-id')
  if (route.params.id === undefined) {
    await router.push({ name: 'chats-id', params: { id: props.chats[0][1][0].id } })
  }

  const { createNewChat } = useCreateChat(BRIDGE_AGENT_ID)

  const currentChatId = computed(() => Number(route.params.id))
  // const allowToAddNewChat = computed(() => !props.currentChatIsEmpty && props.chats?.entries)

  const setActiveChat = (chat: Chat) => {
    router.push({ name: 'chats-id', params: { id: chat.id } })
  }
  const activeChatGroupIndex = computed(() => {
    return props.chats?.findIndex((group) => group[1].find((chat) => chat.id === currentChatId.value))
  })
  const acitveChatIndex = computed(() => {
    let idx = null
    if (!props.chats) {
      return idx
    }
    for (const group of props.chats) {
      for (let i = 0; i < group[1].length; i++) {
        const groupChats = group[1]
        if (groupChats[i].id === currentChatId.value) {
          idx = i
          break
        }
      }
    }
    return idx
  })

  const isNewChat = computed(() => {
    return activeChatGroupIndex.value === 0 && acitveChatIndex.value === 0 && props.currentChatIsEmpty
  })

  const createNewChatHandler = async () => {
    if (isNewChat.value) {
      return
    }

    await createNewChat()
    emits('update-chats')
  }
</script>

<template>
  <div class="chats-history">
    <NewChatButton @click="createNewChatHandler" />
    <div class="chats-list">
      <div
        v-for="[date, group] in chats"
        :key="date"
        class="history-group"
      >
        <div class="history-group__title">{{ date }}</div>
        <div
          v-for="chat in group"
          :key="chat.id"
          :class="['history-item', { active: currentChatId === chat.id }]"
          @click="setActiveChat(chat)"
        >
          Chat #{{ chat.id }}
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .chats-history {
    width: 180px;
    height: 100%;
    padding: 12px 8px;
    border-right: 1px solid var(--border-3);
    font-size: 12px;
    line-height: 17px;

    @include flex(column, flex-start, stretch);
  }

  .history-group {
    @include flex(column);
  }

  .chats-list {
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
    padding: 6px 8px;
    border-radius: 4px;
    color: var(--text-secondary);
    font-weight: 500;

    &:hover,
    &.active {
      background-color: var(--surface-4);
    }
  }
</style>
