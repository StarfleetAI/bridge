<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { PlusIcon } from '~/shared/ui/icons'

  const props = defineProps<{
    disabled: boolean
  }>()
</script>

<template>
  <div :class="['create-task', { disabled: props.disabled }]">
    <PlusIcon />
    Create new
  </div>
</template>

<style lang="scss" scoped>
  .create-task {
    height: 28px;
    padding: 4px 11px 4px 8px;
    border-radius: 4px;
    background-color: var(--button-primary);

    &.disabled {
      opacity: 0.3;
      pointer-events: none;
    }

    @include flex(row, center, center, $gap: 8px);
    @include font-inter-500(14px, 20px, var(--text-on-button));
  }
</style>
