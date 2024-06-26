<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAbilitiesStore } from '~/features/ability'
  import { useAgentsStore, useAgentsNavigation, createAgent, updateAgent } from '~/features/agent'
  import { type Ability } from '~/entities/abilities'
  import { BaseButton } from '~/shared/ui/base'
  import { CrossIcon, SaveIcon, PlusIcon } from '~/shared/ui/icons'
  import { useModalStore } from '~/shared/ui/modal'
  import AbilitiesAddList from './AbilitiesAddList.vue'

  const { disableCreateAgent, isEditAgent } = useAgentsNavigation()

  const { selectedAgent } = useAgentsNavigation()
  const { getById } = useAgentsStore()
  const { abilities } = storeToRefs(useAbilitiesStore())

  const id = ref<number | null>(null)
  const name = ref<string>('')
  const description = ref<string>('')
  const systemMessage = ref<string>('')
  const isCodeInterpreterEnabled = ref(false)
  const isWebBrowserEnabled = ref(false)
  const addedAbilities = ref<Ability[]>([])

  onMounted(async () => {
    if (isEditAgent.value) {
      const agent = getById(selectedAgent.value)
      if (agent) {
        id.value = agent.id
        name.value = agent.name
        description.value = agent.description
        systemMessage.value = agent.system_message
        addedAbilities.value = abilities.value.filter((item) => agent.ability_ids.includes(item.id))
        isCodeInterpreterEnabled.value = agent.is_code_interpreter_enabled
        isWebBrowserEnabled.value = agent.is_web_browser_enabled
      }
    }
  })

  const saveIsEnabled = computed(() => name.value.length > 0)
  const { listAgents } = useAgentsStore()
  const handleSaveAgent = async () => {
    if (isEditAgent.value) {
      await updateAgent({
        id: id.value !== null ? id.value : 0,
        name: name.value,
        description: description.value,
        system_message: systemMessage.value,
        ability_ids: addedAbilities.value.length > 0 ? addedAbilities.value.map((item) => item.id) : [],
        is_code_interpreter_enabled: isCodeInterpreterEnabled.value,
        is_web_browser_enabled: isWebBrowserEnabled.value,
      })
    } else {
      await createAgent({
        name: name.value,
        description: description.value,
        system_message: systemMessage.value,
        ability_ids: addedAbilities.value.length > 0 ? addedAbilities.value.map((item) => item.id) : [],
        is_code_interpreter_enabled: isCodeInterpreterEnabled.value,
        is_web_browser_enabled: isWebBrowserEnabled.value,
      })
    }
    finishCreation()
  }
  const modalStore = useModalStore()

  const openModal = () => {
    modalStore.showModal(AbilitiesAddList, { modelValue: addedAbilities }, (val: unknown) => {
      addedAbilities.value = val as Ability[]
    })
  }

  const finishCreation = () => {
    listAgents()
    disableCreateAgent()
  }
</script>

<template>
  <div class="agent-form">
    <div class="agent-form__header">
      <div class="agent-form__title">{{ isEditAgent ? 'Edit Agent' : 'Create Agent' }}</div>
      <div class="agent-form__actions">
        <BaseButton
          color="blue"
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
      <div class="form-item">
        <label> Code interpreter </label>
        <input
          v-model="isCodeInterpreterEnabled"
          type="checkbox"
        />
      </div>
      <div class="form-item">
        <label> Web browser </label>
        <input
          v-model="isWebBrowserEnabled"
          type="checkbox"
        />
      </div>
    </div>
    <div class="agent-form__abilities">
      <div class="agent-form__abilities-head">
        <div class="agent-form__abilities-head-title">Abilities</div>
        <div
          class="agent-form__abilities-head-add"
          @click="openModal"
        >
          <PlusIcon />Add
        </div>
      </div>
      <div class="agent-form__abilities-list">
        <div
          v-for="ability in addedAbilities"
          :key="ability.id"
          class="agent-form__abilities-list-item"
        >
          <div class="agent-form__abilities-list-item-name">{{ ability.name }}</div>
          <div class="agent-form__abilities-list-item-description">{{ ability.description }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .agent-form {
    &__abilities {
      padding: 24px;

      @include flex(column);
    }

    &__abilities-head {
      margin-bottom: 24px;

      @include flex(row, space-between, center);
    }

    &__abilities-head-title {
      @include font-inter-500(14px, 20px, var(--text-secondary));
    }

    &__abilities-head-add {
      @include flex(row, start, center, 4px);
      @include font-inter-500(14px, 20px, var(--text-tertiary));
    }

    &__abilities-list-item {
      height: 32px;
      border-bottom: 0.5px solid var(--border-3);

      @include flex(row, start, center);
    }

    &__abilities-list-item-name {
      margin-right: 8px;

      @include font-inter-500(14px, 20px, var(--text-secondary));
    }

    &__abilities-list-item-description {
      flex: 1;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;

      @include font-inter-400(12px, 17px, var(--text-tertiary));
    }
  }

  .agent-form__header {
    padding: 10px 24px 13.5px;
    border-bottom: 0.5px solid var(--border-3);

    @include flex(row, space-between, center);
  }

  .agent-form__title {
    @include font-inter-700(14px, 20px, var(--text-secondary));
  }

  .agent-form__actions {
    @include flex(row, flex-end, center, 16px);
  }

  .agent-form__body {
    padding: 26px 24px;
    border-bottom: 0.5px solid var(--border-3);

    @include flex(column, $gap: 24px);
  }

  .form-item {
    label {
      @include font-inter-500(12px, 17px, var(--text-tertiary));
    }

    .input-field {
      height: 40px;
      padding: 8px;
      border: 1px solid var(--border-3);
      border-radius: 6px;
      background: var(--surface-3);
      outline: none;

      @include hide-scrollbar;
      @include font-inter-400(14px, 20px, var(--text-secondary));
    }

    @include flex(column, start, start, $gap: 8px);
  }
</style>
