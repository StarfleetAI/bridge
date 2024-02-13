<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->
<script lang="ts" setup>
  import { useAbilitiesStore } from '@/store/abilities'
  import { useAgentsStore } from '@/store/agents'
  import { useMessagesStore } from '@/store/messages'

  const abilitiesStore = useAbilitiesStore()
  const agentsStore = useAgentsStore()
  const messagesStore = useMessagesStore()

  const loaded = ref(false)

  useHead({
    script: [{ src: 'http://localhost:8098' }]
  })

  onMounted(async () => {
    await nextTick()

    await Promise.all([abilitiesStore.listAbilities(), agentsStore.listAgents()])

    // msgCreatedUnlisten = listen('messages:created', (event) => {
    //   messagesStore.messages.push(event.payload as Message)
    // })

    // msgUpdatedUnlisten = listen('messages:updated', (event) => {
    //   const msg = event.payload as Message
    //   const idx = messagesStore.messages.findIndex((m) => m.id === msg.id)

    //   messagesStore.messages.splice(idx, 1, msg)
    // })

    loaded.value = true
  })

  onBeforeUnmount(async () => {
    // await msgCreatedUnlisten
    // await msgUpdatedUnlisten
  })
</script>

<template>
  <NuxtLayout v-if="loaded">
    <NuxtPage :keepalive="true" />
  </NuxtLayout>
</template>

<style lang="scss"></style>
