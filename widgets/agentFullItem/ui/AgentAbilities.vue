<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { CloudIcon } from '~/shared/ui/icons'
</script>
<template>
  <div class="agent-abilities">
    <div class="agent-abilities__item">
      <div class="agent-abilities__title">
        Test Planning
        <CloudIcon />
      </div>
      <div class="agent-abilities__text">Lirem Lirem Lirem Lirem Lirem Lirem Lirem Lirem Lirem Lirem</div>
    </div>
    <div class="agent-abilities__item">
      <div class="agent-abilities__title">
        Test Planning
        <CloudIcon />
      </div>
      <div class="agent-abilities__text">Lirem Lirem Lirem Lirem Lirem Lirem Lirem Lirem Lirem Lirem</div>
    </div>
    <div class="agent-abilities__item">
      <div class="agent-abilities__title">
        Test Planning
        <CloudIcon />
      </div>
      <div class="agent-abilities__text">Lirem Lirem Lirem Lirem Lirem Lirem Lirem Lirem Lirem Lirem</div>
    </div>
  </div>
</template>
<style scoped lang="scss">
  .agent-abilities {
    &__item {
      margin-bottom: 6px;
      padding: 12px;
      border-radius: 6px;
      background: var(--surface-3);
    }

    &__title {
      gap: 8px;
      margin-bottom: 6px;

      @include flex(row, start, center);
      @include font-inter-500(14px, 20px, var(--text-primary));
    }

    &__text {
      @include font-inter-400(14px, 20px, var(--text-tertiary));
    }
  }
</style>
