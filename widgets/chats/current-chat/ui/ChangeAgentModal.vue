<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAgentsStore } from '~/features/agent'
  import { SearchField } from '~/shared/ui'
  import { BridgeSmallIcon } from '~/shared/ui/icons'
  import { useModalStore } from '~/shared/ui/modal'

  const { agents } = storeToRefs(useAgentsStore())
  const { closeModal } = useModalStore()
  const searchInput = ref('')
</script>

<template>
  <div class="agents-modal">
    <div class="agents-modal__title">Change Agent</div>
    <SearchField
      v-model="searchInput"
      placeholder="Search in Library"
      class="agents-modal__search"
    />
    <div class="agents-list">
      <div
        v-for="agent in agents"
        :key="agent.id"
        class="agents-item"
        @click="closeModal(agent.id)"
      >
        <BridgeSmallIcon
          width="32px"
          height="32px"
        />

        <div class="agents-item__name">
          {{ agent.name }}
        </div>
        <div class="agents-item__description">
          {{ agent.description }}
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .agents-modal {
    width: 100%;
    max-width: 552px;
  }

  .agents-modal__title {
    padding-left: 8px;

    @include font-inter-700(14px, 20px, var(--text-secondary));
  }

  .agents-modal__search {
    margin: 12px 0 16px;
  }

  .agents-list {
    overflow: auto;
    height: 552px;

    @include flex(column, $gap: 6px);
    @include add-scrollbar;
  }

  .agents-item {
    height: 46px;
    padding: 8px 16px;
    border-radius: 6px;
    background-color: var(--surface-2);

    &:hover {
      background-color: var(--surface-3);
    }

    @include flex(row, $align-items: center, $gap: 8px);
  }

  .agents-item__name {
    white-space: nowrap;

    @include font-inter-500(14px, 20px, var(--text-primary));
  }

  .agents-item__description {
    @include text-ellipsis;
    @include font-inter-400(14px, 17px, var(--text-tertiary));
  }
</style>
