<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { PlusIcon } from '~/shared/ui/icons'

  const handleClick = () => {
    navigateTo({ name: 'chats' })
  }
</script>

<template>
  <div
    class="new-chat__btn"
    @click="handleClick"
  >
    <PlusIcon class="new-chat__btn-icon" />New chat
  </div>
</template>

<style lang="scss" scoped>
  .new-chat__btn {
    gap: 6px;

    @include font-inter-400(12px, 17px, var(--button-secondary));
    @include flex(row, center, center);
  }
</style>
