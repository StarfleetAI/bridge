<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAbilitiesStore, useAbilitiesNavigation, createAbility } from '~/features/ability'
  import { BaseButton } from '~/shared/ui/base'
  import { CodeInput } from '~/shared/ui/code-input'
  import { CrossIcon, SaveIcon } from '~/shared/ui/icons'

  const { disableCreateAbility } = useAbilitiesNavigation()

  const name = ref<string>('')
  const description = ref<string>('')
  const code = ref<string>(`def do_something(
    arg1: Annotated[str, "String argument"],
    arg2: Annotated[int, "Integer argument"]
) -> str:
    # Do the actual job here
    return "Something was successful!"`)

  const saveIsEnabled = computed(() => name.value.length > 0)
  const { listAbilities } = useAbilitiesStore()
  const handleSaveAbility = async () => {
    await createAbility({
      name: name.value,
      description: description.value,
      code: code.value,
    })
    finishCreation()
  }

  const finishCreation = () => {
    listAbilities()
    disableCreateAbility()
  }
</script>

<template>
  <div class="ability-form">
    <div class="ability-form__header">
      <div class="ability-form__title">Create Ability</div>
      <div class="ability-form__actions">
        <BaseButton
          type="secondary"
          :disabled="!saveIsEnabled"
          @click="handleSaveAbility"
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
          @click="disableCreateAbility"
        />
      </div>
    </div>
    <div class="ability-form__body">
      <div class="form-item">
        <label>Ability name</label>
        <input
          v-model="name"
          type="text"
          class="input-field"
          placeholder="Ability name"
        />
      </div>
      <div class="form-item">
        <label>Short description</label>
        <textarea
          v-model="description"
          class="input-field"
          placeholder="Short description"
        />
      </div>
      <div class="form-item">
        <CodeInput
          v-model="code"
          label="Code"
        />
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .ability-form {
    @include flex(column);
  }

  .ability-form__header {
    padding: 12px 24px;
    border-bottom: 1px solid var(--border-3);

    @include flex(row, space-between, center);
  }

  .ability-form__title {
    @include font-inter-700(14px, 20px, var(--text-secondary));
  }

  .ability-form__actions {
    @include flex(row, flex-end, center, 16px);
  }

  .ability-form__body {
    padding: 26px 24px;
    border-bottom: 1px solid var(--border-3);

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
      @include font-inter-400(14px, 20px, var(--text-tertiary));
    }

    @include flex(column, start, start, $gap: 8px);
  }

  .prism-editor-wrapper {
    background: var(--surface-3);
  }
</style>
