<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { AbilitiesIcon, CloudIcon } from '~/shared/icons'
  import { Code } from '~/shared/ui/code'
</script>
<template>
  <div class="ability-full-item">
    <div class="ability-full-item__head">
      <div class="ability-full-item__title">
        <AbilitiesIcon />
        <span>Agent #0393</span>
      </div>
    </div>
    <div class="ability-full-item__body">
      <div class="ability-full-item__body-top">
        <div class="ability-full-item__name">
          <div>
            DevOps Specialist
            <CloudIcon />
          </div>
          <div class="ability-full-item__info">by John F.</div>
        </div>
        <div class="ability-full-item__avatar" />
      </div>
      <div class="ability-full-item__result-text">
        This task involves identifying and implementing robust methods to guarantee the security of data at every stage
        of Brand Analytics, from collection through storage to analysis. It is crucial to establish a comprehensive
        approach
      </div>
    </div>
    <div class="ability-full-item__bottom">
      <Code />
    </div>
  </div>
</template>
<style scoped lang="scss">
  .ability-full-item {
    &__head {
      padding: 12px 24px;
      border-bottom: 0.5px solid var(--pill);
    }

    &__title {
      gap: 4px;

      @include flex(row, start, center);
      @include font-inter-500(14px, 22px, var(--text-tertiary));
    }

    &__name {
      gap: 8px;

      & > div {
        gap: 8px;

        @include flex(row, start, center);
      }

      @include flex(column, start, start);
      @include font-inter-700(20px, 28px, var(--text-primary));
    }

    &__info {
      @include font-inter-500(14px, 20px, var(--text-tertiary));
    }

    &__body {
      padding: 24px;
      border-bottom: 0.5px solid var(--pill);
    }

    &__body-top {
      margin-bottom: 16px;

      @include flex(row, space-between, center);
    }

    &__result-text {
      position: relative;

      @include font-inter-400(14px, 19px, var(--text-secondary));
    }

    &__bottom {
      padding: 24px;
    }
  }
</style>
