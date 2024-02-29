<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAgentsStore, useAgentsNavigation, createAgent } from '~/features/agent'
  import { BaseButton } from '~/shared/ui/base'
  import { CrossIcon, SaveIcon } from '~/shared/ui/icons'

  const { disableCreateAgent } = useAgentsNavigation()

  const name = ref<string>('')
  const description = ref<string>('')
  const systemMessage = ref<string>('')

  const saveIsEnabled = computed(() => name.value.length > 0)
  const { listAgents } = useAgentsStore()
  const handleSaveAgent = async () => {
    await createAgent({
      name: name.value,
      description: description.value,
      system_message: systemMessage.value,
      ability_ids: [],
    })
    finishCreation()
  }

  const finishCreation = () => {
    listAgents()
    disableCreateAgent()
  }
</script>

<template>
  <div class="agent-form">
    <div class="agent-form__header">
      <div class="agent-form__title">Create Agent</div>
      <div class="agent-form__actions">
        <BaseButton
          type="secondary"
          :disabled="!saveIsEnabled"
          @click="handleSaveAgent"
        >
          <template #icon>
            <SaveIcon />
          </template>
          Save
        </BaseButton>
        <CrossIcon
          color="#677383"
          height="20px"
          width="20px"
          @click="disableCreateAgent"
        />
      </div>
    </div>
    <div class="agent-form__body">
      <div class="form-item">
        <label>Agent name</label>
        <input
          v-model="name"
          type="text"
          class="input-field"
          placeholder="Agent name"
        />
      </div>
      <div class="form-item">
        <label>Short description</label>
        <input
          v-model="description"
          type="text"
          class="input-field"
          placeholder="Short description"
        />
      </div>
      <div class="form-item">
        <label>Instructions</label>
        <textarea
          v-model="systemMessage"
          class="input-field"
          placeholder="Instructions"
        />
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .agent-form {
    @include flex(column);
  }

  .agent-form__header {
    padding: 12px 24px;
    border-bottom: 1px solid var(--border-3);

    @include flex(row, space-between, center);
  }

  .agent-form__title {
    @include font-inter-700(14px, 20px, var(--text-secondary));
  }

  .agent-form__actions {
    @include flex(row, flex-end, center, 16px);
  }

  .agent-form__body {
    padding: 26px 12px;
    border-bottom: 1px solid var(--border-3);

    @include flex(column, $gap: 8px);
  }

  .form-item {
    @include flex(column, start, start, $gap: 8px);

    label {
      @include font-inter-500(12px, 17px, var(--text-tertiary));
    }

    .input-field {
      border-radius: 6px;
      border: 1px solid var(--border-3);
      background: var(--surface-3);
      padding: 8px;
      outline: none;
      height: 40px;

      @include hide-scrollbar;
      @include font-inter-400(14px, 20px, var(--text-tertiary));
    }
  }
</style>
