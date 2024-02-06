<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<template>
  <div>
    <NuxtLink
      to="/agents/new"
      class="inline-block bg-blue-500 hover:bg-blue-400 text-white py-2 px-4 rounded"
    >
      + Add New
    </NuxtLink>
    <NuxtLink
      to="/agents/abilities"
      class="ml-3 inline-block bg-blue-500 hover:bg-blue-400 text-white py-2 px-4 rounded"
    >
      to Abilities &rarr;
    </NuxtLink>

    <div class="flex mt-3">
      <div
        v-for="agent in agentsStore.agents"
        :key="agent.id"
        class="flex-grow-0 flex-shrink-0 w-1/4"
      >
        <strong>{{ agent.name }}</strong>
        <div class="block text-grey-300">
          {{ agent.description }}
        </div>
        <div class="block text-grey-400 py-2 text-xs">
          <span
            v-for="abilityId in agent.ability_ids"
            :key="abilityId"
            class="inline-block bg-gray-600 rounded-full px-2 py-1 font-semibold text-gray-400 mr-1"
          >
            {{ abilitiesStore.getById(abilityId)?.name }}
          </span>
        </div>
        <a
          href="#"
          class="inline-block bg-blue-500 hover:bg-blue-400 text-white py-0 px-2 rounded"
          @click.prevent="createChat(agent.id)"
        >
          Chat
        </a>
        <NuxtLink
          :to="`/agents/edit?id=${agent.id}`"
          class="inline-block bg-blue-500 hover:bg-blue-400 text-white py-0 px-2 rounded ml-2"
        >
          Edit
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import { useAbilitiesStore } from '@/store/abilities'
  import { useAgentsStore } from '@/store/agents'
  import { useChatsStore } from '@/store/chats'

  const abilitiesStore = useAbilitiesStore()
  const agentsStore = useAgentsStore()
  const chatsStore = useChatsStore()

  const createChat = async (agentId: number) => {
    const chat = await chatsStore.createChat({
      agent_id: agentId
    })

    useRouter().push(`/chats/show?id=${chat.id}`)
  }

  definePageMeta({
    title: 'Agents'
  })
</script>
