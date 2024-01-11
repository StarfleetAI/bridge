<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<template>
  <div>
    <NuxtLink to="/agents/abilities/new" class="inline-block bg-blue-500 hover:bg-blue-400 text-white py-2 px-4 rounded">
      + Add New
    </NuxtLink>
    <NuxtLink to="/agents" class="ml-3 inline-block bg-blue-500 hover:bg-blue-400 text-white py-2 px-4 rounded">
      to Agents &rarr;
    </NuxtLink>
    <ul class="mt-3">
      <li v-for="ability in abilitiesStore.abilities" :key="ability.id">
        <NuxtLink :to="`/agents/abilities/edit?id=${ability.id}`" class="block mb-2">
          <strong>{{ ability.name }}</strong>
          <div class="text-grey-300">
            {{ ability.description }}
          </div>
        </NuxtLink>
      </li>
    </ul>
  </div>
</template>

<script lang="ts" setup>
import { useAbilitiesStore } from '@/store/abilities'
const abilitiesStore = useAbilitiesStore()

definePageMeta({
  title: 'Abilities'
})
</script>
