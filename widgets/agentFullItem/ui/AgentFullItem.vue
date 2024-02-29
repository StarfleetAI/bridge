<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
import AgentAbilities from './AgentAbilities.vue'
</script>
<template>
  <div class="agent-full-item">
    <div class="agent-full-item__title">Agent 0393</div>
    <div class="agent-full-item__body">
      <div class="agent-full-item__avatar" />
      <div class="agent-full-item__name">Cooking</div>
      <div class="agent-full-item__text">
        Enhance research with 200M+ resources and built-in critical reading skills. Access Google Scholar, PubMed,
        JSTOR, Arxiv, and more, effortlessly.
      </div>
      <div class="agent-full-item__info">by Alex Johnson • installed 4,322 times</div>
    </div>
    <div class="agent-full-item__abilities">
      <div class="agent-full-item__abilities-title">Abilities</div>
      <AgentAbilities :abilities="[]" />
    </div>
  </div>
</template>
<style scoped lang="scss">
.agent-full-item {
  &__body {
    padding: 24px;

    @include flex(column, start, center);
  }

  &__title {
    width: 100%;
    padding: 12px 24px;
    border-bottom: 0.5px solid var(--border-3);

    @include font-inter-700(14px, 20px, var(--text-secondary));
    @include flex(row, start, center);
  }

  &__avatar {
    flex-shrink: 0;
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background: var(--text-secondary);
    margin-bottom: 24px;
  }

  &__name {
    margin-bottom: 8px;

    @include font-inter-500(18px, 25px, var(--text-primary));
  }

  &__text {
    margin-bottom: 8px;

    @include font-inter-400(16px, 22px, var(--text-secondary));
  }

  &__info {
    @include font-inter-400(14px, 20px, var(--text-tertiary));
  }
}
</style>
