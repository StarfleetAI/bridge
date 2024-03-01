<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { getSettings, updateSettings } from '~/features/settings'
  import type { Settings } from '~/entities/settings'
  import { BaseButton } from '~/shared/ui/base'
  import { CrossIcon, SaveIcon, SettingsIcon } from '~/shared/ui/icons'
  import FormField from './FormField.vue'

  const settings = ref<Settings>(await getSettings())

  const changedSettings = ref<Settings>(structuredClone(toRaw(settings.value)))

  const settingsChanged = computed(() => {
    return JSON.stringify(changedSettings.value) !== JSON.stringify(settings.value)
  })

  const cancelChanges = () => {
    changedSettings.value = structuredClone(toRaw(settings.value))
  }
  const handleSave = async () => {
    await updateSettings({
      openai_api_key: changedSettings.value.openai_api_key,
      python_path: changedSettings.value.python_path,
      agents: changedSettings.value.agents,
    })

    settings.value = await getSettings()
  }
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
      v-model="changedSettings.openai_api_key"
      name="OpenAI API Key"
      description="Your OpenAI API key"
    />
    <FormField
      v-model="changedSettings.python_path"
      name="Python Path"
      description="The path to your Python interpreter"
    />
    <div class="settings-form__buttons">
      <BaseButton
        :disabled="!settingsChanged"
        type="approve"
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
</style>
