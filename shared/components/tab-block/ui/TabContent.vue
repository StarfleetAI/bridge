<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  const props = defineProps({
    tab: {
      required: true,
      type: [String, Number]
    }
  })

  const activeTab = inject<Ref<number | string>>('activeTab')

  const isActive = computed(() => activeTab?.value === props.tab)
</script>

<template>
  <div
    v-if="isActive"
    class="tab-content"
  >
    <slot />
  </div>
</template>
<style lang="scss" scoped>
  .tab-content {
    position: relative;
    z-index: 2;
    background: #fff;
  }
</style>
