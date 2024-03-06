<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { CrossIcon } from '~/shared/ui/icons'
  import { useModalStore } from '~/shared/ui/modal'

  const modalStore = useModalStore()

  const modalValue = ref(null)
</script>

<template>
  <div
    v-if="modalStore.isVisible"
    class="modal-overlay"
    @click="modalStore.closeModal"
  >
    <div
      class="modal-content"
      @click.stop
    >
      <CrossIcon
        class="close-button"
        :color="'var(--text-tertiary)'"
        width="24"
        height="24"
        @click="modalStore.closeModal(modalValue)"
      />
      <component
        :is="modalStore.contentComponent"
        v-bind="modalStore.contentProps"
        v-model="modalValue"
      />
    </div>
  </div>
</template>

<style scoped lang="scss">
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    z-index: 100;
    width: 100%;
    height: 100%;

    &:after {
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      z-index: 101;
      display: block;
      width: 100%;
      height: 100%;
      background-color: rgba(0, 0, 0, 0.7);
      backdrop-filter: blur(6px);
    }

    @include flex(row, center, center);
  }

  .modal-content {
    position: relative;
    z-index: 102;
    padding: 18px 24px;
    border-radius: 5px;
    background-color: var(--surface-1);
  }

  .close-button {
    position: absolute;
    top: 15px;
    right: 20px;
    border: none;
    background: none;
    cursor: pointer;
  }
</style>
