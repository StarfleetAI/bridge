<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->
<script lang="ts" setup>
  import { useAbilitiesStore } from '@/features/ability'
  import { useAgentsStore } from '@/features/agent'
  import { useDocumentsStore } from '@/features/document'
  import { useDevTools } from '~/shared/lib'
  import { useSettingsStore } from './features/settings'

  useDevTools()

  const { listAbilities } = useAbilitiesStore()
  const { listAgents } = useAgentsStore()
  const { listDocuments } = useDocumentsStore()
  const { getSettings } = useSettingsStore()

  const loaded = ref(false)

  onMounted(async () => {
    await nextTick()
    await Promise.all([listAbilities(), listAgents(), listDocuments(), getSettings()])
    loaded.value = true
  })
</script>

<template>
  <NuxtLayout v-if="loaded">
    <NuxtPage />
  </NuxtLayout>
</template>
