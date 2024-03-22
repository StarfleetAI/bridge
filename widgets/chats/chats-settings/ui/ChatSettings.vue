<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAgentsStore } from '~/features/agent'
  import { useChatsNavigation, useChatsStore } from '~/features/chats'
  import { listModels } from '~/features/models'
  import type { Agent } from '~/entities/agents'
  import { type ChatSettings as ChatSettingsType } from '~/entities/chat'
  import { ModelSelect, type Model } from '~/entities/models'
  import { BRIDGE_AGENT_ID } from '~/shared/lib'
  import { BridgeLargeIcon } from '~/shared/ui/icons'
  import ChatControls from './ChatControls.vue'

  const props = defineProps<{
    settings: ChatSettingsType
  }>()
  const emits = defineEmits<{
    'update-settings': [settings: ChatSettingsType]
  }>()
  const { chatId } = useChatsNavigation()

  const { getById: getChatById } = useChatsStore()
  const { getById: getAgentById } = useAgentsStore()

  const chat = computed(() => getChatById(chatId.value))
  const bridgeAgent = computed(() => getAgentById(BRIDGE_AGENT_ID)!)
  const currentAgent = ref<Agent>(structuredClone(toRaw(bridgeAgent.value)))

  const setChatAgent = () => {
    if (chat.value?.agents_ids?.length === 1) {
      const agent = getAgentById(chat.value?.agents_ids[0])
      if (agent) {
        currentAgent.value = structuredClone(toRaw(agent))
      }
    }
  }
  setChatAgent()

  const models = ref<Model[]>(await listModels())

  const selectedModel = ref(toRaw(props.settings.model_full_name))

  const handleUpdateSettings = () => {
    emits('update-settings', {
      model_full_name: selectedModel.value,
    })
  }

  const handleChangeChat = () => {
    selectedModel.value = toRaw(props.settings.model_full_name)
    setChatAgent()
  }
  watch(
    () => chat.value,
    () => {
      handleChangeChat()
    },
    {
      deep: true,
    },
  )
</script>

<template>
  <div class="chat-settings">
    <div class="chat-settings__head">
      <div class="chat-settings__title">
        <b>Chat Settings</b>
      </div>

      <ChatControls :chat="chat" />
    </div>
    <div class="chat-settings__agent-wrapper">
      <BridgeLargeIcon />
      <div class="agent__name">
        {{ currentAgent.name }}
      </div>
      <div
        v-if="currentAgent.description"
        class="agent__description"
      >
        {{ currentAgent.description }}
      </div>
      <div class="agent__author">by StarfleetAI</div>
    </div>
    <div class="chat-settings__fields">
      <div class="chat-settings__field">
        <div class="field-name">Model</div>
        <div class="field-input">
          <ModelSelect
            v-model="selectedModel"
            :models="models"
            @update:model-value="handleUpdateSettings"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .chat-settings {
    position: relative;
    z-index: 3;

    @include flex(column);
  }

  .chat-settings__head {
    height: 56px;
    padding: 12px 24px;
    border-bottom: 0.5px solid var(--border-3);

    @include flex(row, space-between, center);
  }

  .chat-settings__title {
    b {
      color: var(--text-secondary);
    }

    @include font-inter-400(14px, 20px, var(--text-tertiary));
    @include flex(row, start, center, 8px);
  }

  .chat-settings__agent-wrapper {
    padding: 24px 0;
    border-bottom: 0.5px solid var(--border-3);

    @include flex(column, center, center, 16px);
  }

  .agent__name {
    @include font-inter-700(20px, 28px, var(--text-secondary));
  }

  .agent__description {
    text-align: center;

    @include font-inter-400(16px, 22px, var(--text-secondary));
  }

  .agent__author {
    @include font-inter-400(12px, 17px, var(--text-tertiary));
  }

  .chat-settings__fields {
    padding: 24px;

    @include flex(column, flex-start, center, 40px);
  }

  .chat-settings__field {
    width: 100%;
    max-width: 260px;

    @include flex(column, $gap: 8px);
  }

  .field-name {
    @include font-inter-500(12px, 17px, var(--text-tertiary));
  }
</style>
