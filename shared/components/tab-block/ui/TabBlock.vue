<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  const activeTab = ref<number | string>('tab1')
  const setActiveTab = (tab: number | string) => {
    activeTab.value = tab
  }

  provide('activeTab', activeTab)
  provide('setActiveTab', setActiveTab)
</script>

<template>
  <div class="tab-block">
    <slot />
  </div>
</template>
<style lang="scss" scoped>
  .tab-block {
    border-radius: 12px;
    background: var(--surface-2);
  }
</style>
