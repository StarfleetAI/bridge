<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { listModels } from '~/features/models'
  import { useSettingsStore } from '~/features/settings'
  import { ModelSelect } from '~/entities/models'
  import type { Settings } from '~/entities/settings'
  import { BaseButton } from '~/shared/ui/base'
  import { CrossIcon, SaveIcon, SettingsIcon } from '~/shared/ui/icons'
  import FormField from './FormField.vue'
  const { getSettings, updateSettings } = useSettingsStore()
  await getSettings()
  const { settings } = storeToRefs(useSettingsStore())
  const changedSettings = ref<Settings>(structuredClone(toRaw(settings.value!)))

  const settingsChanged = computed(() => {
    return JSON.stringify(changedSettings.value) !== JSON.stringify(settings.value)
  })

  const cancelChanges = () => {
    changedSettings.value = structuredClone(toRaw(settings.value!))
  }
  const handleSave = () => {
    updateSettings({
      api_keys: changedSettings.value.api_keys,
      agents: changedSettings.value.agents,
      default_model: changedSettings.value.default_model,
    })
  }
  const models = ref(await listModels())
  const providers = ref(new Set(models.value.map((model) => model.provider)))
</script>

<template>
  <div class="settings-form">
    <div class="settings-form__title">
      <SettingsIcon
        width="20"
        height="20"
      />General Settings
    </div>

    <FormField
      v-for="provider in providers"
      v-model.trim="changedSettings.api_keys[provider]"
      :name="`${provider} API Key`"
      :description="`Your ${provider} API Key`"
    />
    <div class="select-field">
      <div class="select-field__info">
        <div class="select-field__name">Default model</div>
        <div class="select-field__description">The default model to use</div>
      </div>
      <ModelSelect
        v-model.trim="changedSettings.default_model"
        :models="models"
        class="select-field__input"
      />
    </div>

    <div class="settings-form__buttons">
      <BaseButton
        :disabled="!settingsChanged"
        color="green"
        @click="handleSave"
      >
        <template #icon>
          <SaveIcon />
        </template>
        Save
      </BaseButton>
      <BaseButton
        outlined
        @click="cancelChanges"
      >
        <template #icon>
          <CrossIcon />
          Cancel
        </template>
      </BaseButton>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .settings-form {
    flex: 1;
    padding: 24px;

    @include flex(column, $gap: 32px);
  }

  .settings-form__title {
    @include flex($align-items: center, $gap: 8px);
  }

  .settings-form__buttons {
    margin-left: 198px;

    @include flex($gap: 16px);
  }

  .select-field {
    width: 100%;
    height: 40px;

    @include flex($gap: 16px);
  }

  .select-field__info {
    flex-shrink: 0;
    width: 182px;
    text-align: end;

    @include flex(column, $align-items: flex-end, $gap: 4px);
  }

  .select-field__name {
    @include font-inter-700(12px, 17px, var(--text-secondary));
  }

  .select-field__description {
    word-break: break-word;

    @include font-inter-400(12px, 17px, var(--text-tertiary));
  }

  .select-field__input {
    width: 300px;
  }
</style>
