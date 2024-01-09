<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<template>
  <div v-if="loaded" class="flex h-screen p-3 space-x-3">
    <!-- Sidebar -->
    <div class="bg-gray-800 rounded-2xl py-2 px-2 flex flex-col items-center justify-start space-y-2">
      <NuxtLink to="/" class="w-11 h-11 bg-gray-500 flex items-center justify-center rounded-xl font-bold">
        A
      </NuxtLink>
      <NuxtLink to="/docs" class="w-11 h-11 bg-gray-500 flex items-center justify-center rounded-xl font-bold">
        D
      </NuxtLink>
      <NuxtLink to="/settings" class="w-11 h-11 bg-gray-500 flex items-center justify-center rounded-xl font-bold">
        S
      </NuxtLink>
    </div>

    <!-- Content Area -->
    <div class="flex flex-col flex-grow">
      <!-- eslint-disable-next-line vue/no-v-html -->
      <h1 class="text-2xl font-bold mb-3 text-slate-400" v-html="$route.meta.title" />

      <div class="bg-gray-700 rounded-2xl p-3 flex-grow">
        <NuxtPage />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import type { UnlistenFn } from '@tauri-apps/api/event'
import { listen } from '@tauri-apps/api/event'

import type { Message } from '@/store/messages'
import { useMessagesStore } from '@/store/messages'
import { useAbilitiesStore } from '@/store/abilities'
import { useAgentsStore } from '@/store/agents'

const abilitiesStore = useAbilitiesStore()
const agentsStore = useAgentsStore()
const messagesStore = useMessagesStore()

const loaded = ref(false)
let msgCreatedUnlisten: Promise<UnlistenFn>
let msgUpdatedUnlisten: Promise<UnlistenFn>

onMounted(async () => {
  await nextTick()

  await Promise.all([
    abilitiesStore.listAbilities(),
    agentsStore.listAgents()
  ])

  msgCreatedUnlisten = listen('messages:created', (event) => {
    messagesStore.messages.push(event.payload as Message)
  })

  msgCreatedUnlisten = listen('messages:updated', (event) => {
    const msg = event.payload as Message
    const idx = messagesStore.messages.findIndex(m => m.id === msg.id)

    messagesStore.messages.splice(idx, 1, msg)
  })

  loaded.value = true
})

onBeforeUnmount(async () => {
  await msgCreatedUnlisten
  await msgUpdatedUnlisten
})
</script>
