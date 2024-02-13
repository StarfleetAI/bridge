<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { abilitiesInjectionKey } from '~/features/abilities/list-abilities'
  import { type Ability } from '~/entities/ability'
  import { type Message, type ToolCall } from '~/entities/chat'

  const props = defineProps<{
    message: Message
  }>()
  const abilities = inject(abilitiesInjectionKey)

  const ability = computed(() => {
    const { tool_calls } = props.message
    let toolCalls: ToolCall[] = []

    if (tool_calls) {
      try {
        toolCalls = JSON.parse(tool_calls)
      } catch (error) {
        console.error(error)
      }
    }
    return abilities?.value.find((item) => {
      let parsedParameters: Record<string, unknown> = {}
      try {
        parsedParameters = JSON.parse(item.parameters_json)
      } catch (error) {
        console.error(error)
      }
      return parsedParameters.name === toolCalls[0].function.name
    }) as Ability
  })
</script>

<template>
  <div>{{ ability?.name }}</div>
</template>

<style lang="scss" scoped></style>
