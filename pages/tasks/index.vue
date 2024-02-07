<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  definePageMeta({
    title: 'Tasks',
    layout: 'only-content'
  })
</script>

<template>
  <div class="flex flex-col">
    <NuxtLink
      class=""
      to="/"
    >
      &larr; Go back
    </NuxtLink>
    Tasks
  </div>
</template>

<style lang="scss" scoped>
  div {
    color: var(--text-primary);
  }
</style>
