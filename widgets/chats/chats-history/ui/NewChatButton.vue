<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { PlusIcon } from '~/shared/ui/icons'
</script>

<template>
  <div class="new-chat__btn"><PlusIcon class="new-chat__btn-icon" />New chat</div>
</template>

<style lang="scss" scoped>
  .new-chat__btn {
    gap: 8px;
    align-self: center;
    width: 148px;
    height: 32px;
    margin-bottom: 40px;
    padding: 6px 10px;
    border: 1px solid var(--text-tertiary);
    border-radius: 4px;

    @include font-inter-500(14px, 20px, var(--text-tertiary));
    @include flex(row, center, center);
  }
</style>
