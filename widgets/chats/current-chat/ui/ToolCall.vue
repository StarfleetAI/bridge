<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAbilitiesStore } from '~/features/abilities'
  import { useMessagesStore } from '~/features/chats'
  import { type Ability } from '~/entities/ability'
  import { Status, type ToolCall } from '~/entities/chat'
  import { ChatLoader } from '~/shared/ui/base'
  import { CheckIcon, CodeIcon, CrossIcon } from '~/shared/ui/icons'

  const props = defineProps<{
    toolCall: ToolCall
    status: Status
    messageId: number
  }>()
  const { abilities } = storeToRefs(useAbilitiesStore())
  const { approveToolCall, denyToolCall } = useMessagesStore()
  const ability = computed(() => {
    return abilities?.value?.find((item) => {
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
        value: parsedArgs[param]
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
        arguments: parsedFunctionArguments.value
      }
    }
  })
  const showActions = computed(() => {
    return props.status === Status.WAITING_FOR_TOOL_CALL
  })
  const isProcessing = computed(() => {
    return props.status === Status.WRITING
  })
</script>

<template>
  <div :class="['tool', { done: status === Status.COMPLETED }]">
    <CodeIcon class="tool__icon" />
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
        v-if="showActions && !isProcessing"
        class="tool__parameters"
      >
        <div
          v-for="{ name, description, value } in fullAction.function.arguments"
          :key="name"
          class="tool__parameters-item"
        >
          <div class="tool__parameters-item-name">
            <div class="tool__parameter-name">{{ name }}</div>
            <div class="tool__parameter-description">{{ description }}</div>
          </div>
          <div class="tool__parameters-item-value">
            {{ value }}
          </div>
        </div>
      </div>
      <div
        v-if="showActions"
        class="tool__actions"
      >
        <div
          class="tool__btn approve"
          @click="approveToolCall(Number(messageId))"
        >
          <CheckIcon />
          Approve
        </div>
        <div
          class="tool__btn deny"
          @click="denyToolCall(Number(messageId))"
        >
          <CrossIcon />
          Deny
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

    @include flex(row);
  }

  .tool__icon {
    flex-shrink: 0;
  }

  .tool__name {
    height: 22px;
    padding-top: 1px;

    @include font-inter-700(14px, 20px);
  }

  .tool__description {
    margin-top: 8px;

    @include font-inter-500(14px, 20px, var(--text-tertiary));
  }

  .tool__content {
    flex: 1 0;

    // gap: 16px;
  }

  .tool__loading {
    gap: 8px;
    margin-top: 18px;

    @include font-inter-400(14px, 20px, var(--button-primary));
    @include flex(row, flex-start, center);
  }

  .tool__parameters {
    width: 100%;
    margin-top: 16px;
    border-radius: 6px;
    background-color: var(--surface-3);

    @include flex(column);
  }

  .tool__parameters-item {
    gap: 8px;
    width: 100%;

    // min-height: 56px;
    padding: 8px 12px;

    &:not(:last-child) {
      border-bottom: 1px solid var(--border-3);
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
  }

  .tool__parameters-item-value {
    font-weight: 500;
    font-size: 12px;
    line-height: 17px;
    white-space: pre-wrap;

    @include font-mono;
  }

  .tool__actions {
    gap: 16px;
    margin-top: 16px;
    cursor: default;
    user-select: none;

    @include flex(row);
  }

  .tool__btn {
    gap: 8px;
    width: 120px;
    height: 36px;
    border-radius: 4px;

    &.approve {
      background-color: var(--status-done);
    }

    &.deny {
      background-color: var(--status-failed);
    }

    @include font-inter-500(14px, 20px, var(--text-on-button));
    @include flex(row, center, center);
  }
</style>
