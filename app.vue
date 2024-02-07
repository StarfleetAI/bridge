<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->
<script lang="ts" setup>
  import type { UnlistenFn } from '@tauri-apps/api/event'
  import { listen } from '@tauri-apps/api/event'

  import { useAbilitiesStore } from '@/store/abilities'
  import { useAgentsStore } from '@/store/agents'
  import type { Message } from '@/store/messages'
  import { useMessagesStore } from '@/store/messages'

  const abilitiesStore = useAbilitiesStore()
  const agentsStore = useAgentsStore()
  const messagesStore = useMessagesStore()

  const loaded = ref(false)
  let msgCreatedUnlisten: Promise<UnlistenFn>
  let msgUpdatedUnlisten: Promise<UnlistenFn>

  onMounted(async () => {
    await nextTick()

    await Promise.all([abilitiesStore.listAbilities(), agentsStore.listAgents()])

    msgCreatedUnlisten = listen('messages:created', (event) => {
      messagesStore.messages.push(event.payload as Message)
    })

    msgUpdatedUnlisten = listen('messages:updated', (event) => {
      const msg = event.payload as Message
      const idx = messagesStore.messages.findIndex((m) => m.id === msg.id)

      messagesStore.messages.splice(idx, 1, msg)
    })

    loaded.value = true
  })

  onBeforeUnmount(async () => {
    await msgCreatedUnlisten
    await msgUpdatedUnlisten
  })
</script>

<template>
  <NuxtLayout v-if="loaded">
    <NuxtPage />
  </NuxtLayout>
</template>

<style lang="scss">
  * {
    box-sizing: border-box;
  }

  html {
    font-family: Inter, sans-serif;
  }

  body {
    margin: 0;
    padding: 0;
    background-color: var(--surface-1);
  }
</style>
