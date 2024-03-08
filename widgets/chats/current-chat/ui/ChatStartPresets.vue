<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  defineEmits<{
    select: [preset: string]
  }>()

  const startPresets = [
    'How to improve this interface?',
    'How to make the design more friendly?',
    'Need advice on the color scheme',
    'Any ideas for animation?',
  ]
</script>

<template>
  <div class="presets">
    <div
      v-for="preset in startPresets"
      class="presets-item"
      @click="$emit('select', preset)"
    >
      {{ preset }}
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .presets {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    grid-auto-flow: row;
    gap: 12px;
  }

  .presets-item {
    overflow: hidden;
    padding: 12px;
    border-radius: 6px;
    background-color: var(--surface-2);

    &:hover {
      background-color: var(--surface-3);
    }

    @include text-ellipsis;
    @include font-inter-400(16px, 22px, var(--text-secondary));
  }
</style>
