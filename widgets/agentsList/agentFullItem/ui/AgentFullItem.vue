<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { AgentIcon, AbilitiesIcon, CloudIcon } from '~/shared/ui/icons'
  import AgentAbilities from './AgentAbilities.vue'
</script>
<template>
  <div class="agent-full-item">
    <div class="agent-full-item__head">
      <div class="agent-full-item__title">
        <AgentIcon />
        <span>Agent #0393</span>
      </div>
    </div>
    <div class="agent-full-item__body">
      <div class="agent-full-item__body-top">
        <div class="agent-full-item__name">
          <div>
            DevOps Specialist
            <CloudIcon />
          </div>
          <div class="agent-full-item__info">by John F. • installed 54,322 times</div>
        </div>
        <div class="agent-full-item__avatar" />
      </div>
      <div class="agent-full-item__result-text">
        This task involves identifying and implementing robust methods to guarantee the security of data at every stage
        of Brand Analytics, from collection through storage to analysis. It is crucial to establish a comprehensive
        approach that safeguards sensitive information and maintains the integrity and confidentiality of the data
        processed by the neural network. Below is a list of key actions and considerations for ensuring data security
      </div>
    </div>
    <div class="agent-full-item__abilities">
      <div class="agent-full-item__abilities-title">
        <AbilitiesIcon />
        Abilities
      </div>
      <AgentAbilities />
    </div>
  </div>
</template>
<style scoped lang="scss">
  .agent-full-item {
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

    &__abilities {
      padding: 24px;
    }

    &__abilities-title {
      gap: 5px;
      margin-bottom: 32px;

      @include flex(row, start, center);
      @include font-inter-500(14px, 20px, var(--text-tertiary));
    }

    &__avatar {
      flex-shrink: 0;
      width: 48px;
      height: 48px;
      border-radius: 50%;
      background: #d9d9d9;
    }

    &__result-text {
      position: relative;

      &:after {
        content: '';
        position: absolute;
        right: 0;
        bottom: 0;
        left: 0;
        height: 50px;
        background: linear-gradient(to bottom, rgba(36, 39, 49, 0), rgba(36, 39, 49, 1));
      }

      @include font-inter-400(14px, 19px, var(--text-secondary));
    }
  }
</style>
