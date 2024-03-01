<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { highlight, languages } from 'prismjs'
  import 'prismjs/components/prism-python'
  import 'prismjs/themes/prism-tomorrow.css'
  import { PrismEditor } from 'vue-prism-editor'
  import 'vue-prism-editor/dist/prismeditor.min.css'

  import { useAbilitiesStore, type UpdateAbility } from '~/features/ability'

  definePageMeta({
    title: 'Abilities &raquo; Edit',
  })

  const abilitiesStore = useAbilitiesStore()

  const highlighter = (code: string) => {
    return highlight(code, languages.python, 'python')
  }

  const route = useRoute()
  const router = useRouter()

  const ability = computed(() => {
    return abilitiesStore.getById(Number(route.query.id))
  })

  const req = ref<UpdateAbility>({
    id: ability.value?.id || 0,
    name: ability.value?.name || '',
    description: ability.value?.description || '',
    code: ability.value?.code || '',
  })

  const updateAbility = async () => {
    await abilitiesStore.updateAbility(req.value)
    router.push('/agents/abilities')
  }
</script>

<template>
  <div class="max-w-6xl mx-auto py-10">
    <NuxtLink
      to="/agents/abilities"
      class="text-blue-400 hover:text-blue-300"
    >
      ← Back to Abilities
    </NuxtLink>
    <form
      class="mt-8"
      @submit.prevent="updateAbility"
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
          required="true"
          class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-3 text-gray-700"
        />
      </div>
      <div class="mb-6">
        <label
          for="code"
          class="block text-sm font-medium text-gray-200 mb-2"
          >Code</label
        >
        <prism-editor
          v-model="req.code"
          :highlight="highlighter"
          line-numbers
          class="bg-gray-800 rounded-lg px-3 py-5 font-mono text-sm"
          :tab-size="4"
        />
      </div>
      <button
        type="submit"
        class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
      >
        Save
      </button>
    </form>
  </div>
</template>
