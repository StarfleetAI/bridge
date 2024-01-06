<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<template>
  <div>
    <NuxtLink to="/agents/new" class="inline-block bg-blue-500 hover:bg-blue-400 text-white py-2 px-4 rounded">
      + Add New
    </NuxtLink>
    <NuxtLink to="/agents/abilities" class="ml-3 inline-block bg-blue-500 hover:bg-blue-400 text-white py-2 px-4 rounded">
      to Abilities &rarr;
    </NuxtLink>
    <ul>
      <li v-for="agent in agentsStore.agents" :key="agent.id">
        <NuxtLink :to="`/agents/edit?id=${agent.id}`" class="block mb-2">
          <strong>{{ agent.name }}</strong>
          <div class="block text-grey-300">
            {{ agent.description }}
          </div>
          <div class="block text-grey-400">
            <span
              v-for="abilityId in agent.ability_ids"
              :key="abilityId"
              class="inline-block bg-gray-600 rounded-full px-3 py-1 text-sm font-semibold text-gray-400 mr-2"
            >
              {{ abilitiesStore.getById(abilityId)?.name }}
            </span>
          </div>
        </NuxtLink>
      </li>
    </ul>
  </div>
</template>

<script lang="ts" setup>
import { useAbilitiesStore } from '@/store/abilities'
import { useAgentsStore } from '@/store/agents'

const abilitiesStore = useAbilitiesStore()
const agentsStore = useAgentsStore()

definePageMeta({
  title: 'Agents'
})
</script>
