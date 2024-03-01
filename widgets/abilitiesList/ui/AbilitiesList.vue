<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { AbilityItem, type Ability } from '~/entities/abilities'

  defineProps<{
    abilities: Ability[]
  }>()
</script>

<template>
  <div class="abilities-list">
    <AbilityItem
      v-for="ability in abilities"
      :key="ability.id"
      :ability="ability"
    />
  </div>
</template>
<style scoped lang="scss">
  .abilities-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 12px;
    padding: 16px 24px;
  }
</style>
