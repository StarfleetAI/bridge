<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { SettingsForm } from '~/widgets/settingsForm'

  import { BaseContainer } from '~/shared/ui/base'

  definePageMeta({
    title: 'Settings',
  })
</script>

<template>
  <BaseContainer>
    <template #main>
      <SettingsForm />
    </template>
  </BaseContainer>
</template>
