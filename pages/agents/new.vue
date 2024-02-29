<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAgentsStore, type CreateAgent } from '@/features/agent'
  import { useAbilitiesStore } from '~/features/abilities'

  definePageMeta({
    title: 'Agents &raquo; New',
  })

  const abilitiesStore = useAbilitiesStore()
  const agentsStore = useAgentsStore()

  const req = ref<CreateAgent>({
    name: '',
    description: '',
    system_message: '',
    ability_ids: [],
  })
  const router = useRouter()

  const createAgent = async () => {
    await agentsStore.createAgent(req.value)
    router.push('/agents')
  }
</script>
<template>
  <div class="max-w-2xl mx-auto py-10">
    <NuxtLink
      to="/agents"
      class="text-blue-400 hover:text-blue-300"
    >
      ← Back to Agents
    </NuxtLink>
    <form
      class="mt-8"
      @submit.prevent="createAgent"
    >
      <div class="mb-6">
        <label
          for="name"
          class="block text-sm font-medium text-gray-200 mb-2"
          >Name</label
        >
        <input
          id="name"
          v-model="req.name"
          required="true"
          class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-3 text-gray-700"
        />
      </div>
      <div class="mb-6">
        <label
          for="description"
          class="block text-sm font-medium text-gray-200 mb-2"
          >Description</label
        >
        <input
          id="description"
          v-model="req.description"
          class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-3 text-gray-700"
        />
      </div>
      <div class="mb-6">
        <label
          for="code"
          class="block text-sm font-medium text-gray-200 mb-2"
          >System Message</label
        >
        <textarea
          id="code"
          v-model="req.system_message"
          class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border border-gray-300 rounded-md p-3 h-96 font-mono text-gray-700"
          required="true"
        />
      </div>
      <div class="mb-6">
        <label
          for="code"
          class="block text-sm font-medium text-gray-200 mb-2"
          >Abilities</label
        >
        <select
          v-model="req.ability_ids"
          multiple
          class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border border-gray-300 rounded-md p-3 h-96 font-mono text-gray-700"
        >
          <option
            v-for="ability in abilitiesStore.abilities"
            :key="ability.id"
            :value="ability.id"
          >
            {{ ability.name }}
          </option>
        </select>
      </div>
      <button
        type="submit"
        class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
      >
        Create
      </button>
    </form>
  </div>
</template>
