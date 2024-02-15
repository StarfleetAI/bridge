<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->
<script lang="ts" setup>
  import { useAbilitiesStore } from '@/features/abilities'
  import { useAgentsStore } from '@/features/agents'

  const abilitiesStore = useAbilitiesStore()
  const agentsStore = useAgentsStore()

  const loaded = ref(false)

  useHead({
    script: [{ src: 'http://localhost:8098' }]
  })

  onMounted(async () => {
    await nextTick()
    await Promise.all([abilitiesStore.listAbilities(), agentsStore.listAgents()])
    loaded.value = true
  })
</script>

<template>
  <NuxtLayout v-if="loaded">
    <NuxtPage />
  </NuxtLayout>
</template>

<style lang="scss"></style>
