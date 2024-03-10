<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useDocumentsStore, useDocumentsNavigation, createDocument } from '~/features/document'
  import { BaseButton } from '~/shared/ui/base'
  import { CrossIcon, SaveIcon } from '~/shared/ui/icons'

  const { disableCreateDocument } = useDocumentsNavigation()

  const title = ref<string>('')
  const saveIsEnabled = computed(() => title.value.length > 0)
  const { listDocuments } = useDocumentsStore()
  const handleSaveDocument = async () => {
    await createDocument({
      title: title.value,
    })
    finishCreation()
  }
  const finishCreation = () => {
    listDocuments()
    disableCreateDocument()
  }
</script>

<template>
  <div class="agent-form">
    <div class="agent-form__header">
      <div class="agent-form__title">Create Document</div>
      <div class="agent-form__actions">
        <BaseButton
          type="secondary"
          :disabled="!saveIsEnabled"
          @click="handleSaveDocument"
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
          @click="disableCreateDocument"
        />
      </div>
    </div>
    <div class="agent-form__body">
      <div class="form-item">
        <label>Document title</label>
        <input
          v-model="title"
          type="text"
          class="input-field"
          placeholder="Document title"
        />
      </div>
      <div class="form-item">
        <label>Short description</label>
        <input
          type="text"
          class="input-field"
          placeholder="Short description"
        />
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
      @include font-inter-400(14px, 20px, var(--text-secondary));
    }

    @include flex(column, start, start, $gap: 8px);
  }
</style>
