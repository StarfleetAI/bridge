<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAbilitiesStore } from '~/features/ability'

  import { type Ability } from '~/entities/abilities'
  import type { Agent } from '~/entities/agents'
  import { Status, type ToolCall } from '~/entities/chat'
  import { ChatLoader } from '~/shared/ui/base'
  import { ChevronDownIcon, CubeIcon } from '~/shared/ui/icons'

  const props = defineProps<{
    toolCall: ToolCall
    status: Status
    messageId: number
    currentAgent: Agent
  }>()
  const { abilities } = storeToRefs(useAbilitiesStore())
  const agentAbilities = computed(() => {
    return abilities?.value?.filter((item) => props.currentAgent.ability_ids.includes(item.id))
  })
  const ability = computed(() => {
    return agentAbilities?.value?.find((item) => {
      let parsedParameters: Record<string, unknown> = {}
      try {
        parsedParameters = JSON.parse(item.parameters_json)
      } catch (error) {
        console.error(error)
      }
      return parsedParameters.name === props.toolCall.function.name
    }) as Ability
  })
  const parsedFunctionArguments = computed(() => {
    try {
      const parsedArgs = JSON.parse(props.toolCall.function.arguments)
      const abilityParams = JSON.parse(ability.value.parameters_json)

      if (!abilityParams) {
        return {}
      }

      return Object.keys(abilityParams.parameters.properties).map((param) => ({
        name: param,
        description: abilityParams.parameters.properties[param].description,
        value: parsedArgs[param],
      }))
    } catch {
      return {}
    }
  })
  const fullAction = computed(() => {
    return {
      ...props.toolCall,
      ...ability.value,
      function: {
        name: props.toolCall.function.name,
        arguments: parsedFunctionArguments.value,
      },
    }
  })

  const isProcessing = computed(() => {
    return props.status === Status.WRITING
  })

  const showMore = ref(false)
  const parametersWrapperRef = ref<HTMLElement>()
  const toggleShowMore = () => {
    showMore.value = !showMore.value
  }

  const parametersListRef = ref<HTMLElement[]>([])

  const showMoreButtonIsVisible = computed(() => {
    return parametersListRef.value.some((item) => item.scrollHeight > item.clientHeight)
  })

  const showMoreButtonText = computed(() => {
    return showMore.value ? 'Less' : 'More'
  })
</script>

<template>
  <div :class="['tool', { done: status === Status.COMPLETED, denied: status === Status.TOOL_CALL_DENIED }]">
    <CubeIcon class="tool__icon" />
    <div class="tool__content">
      <div class="tool__name">{{ ability?.name }}</div>
      <div class="tool__description">{{ ability?.description }}</div>
      <div
        v-if="isProcessing"
        class="tool__loading"
      >
        <ChatLoader />
        Processing
      </div>
      <div
        v-if="!isProcessing"
        ref="parametersWrapperRef"
        :class="['tool__parameters', { full: showMore }]"
      >
        <div
          v-for="{ name, description, value } in fullAction.function.arguments"
          :key="name"
          class="tool__parameters-item"
        >
          <div class="tool__parameters-item-name">
            <div class="tool__parameter-name">{{ name }}</div>
            <div class="tool__parameter-description">
              {{ description }}
            </div>
          </div>
          <div
            ref="parametersListRef"
            class="tool__parameters-item-value"
          >
            {{ value }}
          </div>
        </div>
        <div
          v-if="showMoreButtonIsVisible"
          :class="['tool__show-more', { visible: showMore }]"
          @click="toggleShowMore"
        >
          {{ showMoreButtonText }}
          <ChevronDownIcon />
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .tool {
    gap: 8px;
    padding: 16px 12px;
    border: 1px solid transparent;
    border-radius: 6px;
    background-color: var(--surface-2);

    &.done {
      border-left: 2px solid var(--status-done);
    }

    &.denied {
      border-left: 2px solid var(--status-failed);
    }

    @include flex(row);
  }

  .tool__icon {
    flex-shrink: 0;
    color: var(--text-secondary);
  }

  .tool__name {
    height: 22px;
    padding-top: 1px;

    @include font-inter-700(16px, 22px, var(--text-secondary));
  }

  .tool__description {
    margin-top: 8px;

    @include font-inter-400(16px, 22px, var(--text-tertiary));
  }

  .tool__content {
    flex: 1 0;
  }

  .tool__loading {
    gap: 8px;
    margin-top: 18px;

    @include font-inter-400(14px, 20px, var(--button-primary));
    @include flex(row, flex-start, center);
  }

  .tool__parameters {
    overflow: hidden;
    width: 100%;
    max-height: 200px;
    margin-top: 16px;
    border-radius: 6px;
    background-color: var(--surface-3);
    transition: all 0.2s ease;

    &.full {
      max-height: 5000px;

      & .tool__parameters-item-value,
      & .tool__parameters-item-description {
        display: block;
      }
    }

    @include flex(column);
  }

  .tool__parameters-item {
    gap: 8px;
    width: 100%;
    padding: 8px 12px;

    &:not(:first-child) {
      border-top: 1px solid var(--border-3);
    }

    @include flex(row);
  }

  .tool__parameters-item-name {
    flex-shrink: 0;
    gap: 6px;
    width: 158px;

    @include flex(column);
  }

  .tool__parameter-name {
    font-weight: 500;
    font-size: 12px;
    line-height: 17px;

    @include font-mono;
  }

  .tool__parameter-description {
    @include font-inter-400(12px, 17px, var(--text-tertiary));
    @include line-clamp(2);
  }

  .tool__parameters-item-value {
    font-weight: 500;
    font-size: 12px;
    line-height: 17px;
    white-space: pre-wrap;
    cursor: auto;
    user-select: initial;

    @include line-clamp(3);
    @include font-mono;
  }

  .tool__show-more {
    gap: 4px;
    margin-bottom: 8px;
    margin-left: 178px;

    &.visible {
      & svg {
        transform: rotate(180deg);
      }
    }

    @include font-inter-400(12px, 17px, var(--text-tertiary));
    @include flex(row, flex-start, center);
  }
</style>
