<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { NoAvatarIcon } from '~/shared/ui/icons'
</script>

<template>
  <NuxtLink
    to="/"
    class="user-avatar"
  >
    <NoAvatarIcon
      width="24"
      height="24"
    />
  </NuxtLink>
</template>

<style lang="scss" scoped>
  .user-avatar {
    width: 24px;
    height: 24px;
    border-radius: 50%;
  }
</style>
