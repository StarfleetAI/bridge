<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { BRIDGE_AGENT_ID } from '~/shared/lib'
  import { BridgeSmallIcon, NoAvatarIcon } from '~/shared/ui/icons'

  const props = defineProps<{
    agents: { name: string; id: number }[]
  }>()

  const containerWidth = computed(() => {
    return `${(props.agents.length - 1) * 16 + 24}px`
  })
  // const goToAgent = (id: number) => {
  //   navigateTo({ name: 'agents', query: { agent: id } })
  // }
</script>

<template>
  <div class="avatar-list">
    <template v-if="agents.length === 1">
      <div class="avatar-list__single">
        <div class="avatar-list__name">{{ agents[0].name }}</div>
        <!-- <template v-if="agents[0].avatar">
          <img
            :src="agents[0].avatar"
            alt="avatar"
            class="avatar-list__avatar"
          />
        </template> -->
        <!-- <template v-else> -->
        <BridgeSmallIcon v-if="agents[0].id === BRIDGE_AGENT_ID" />
        <NoAvatarIcon
          v-else
          class="avatar-list__avatar"
        />
        <!-- </template> -->
      </div>
    </template>
    <template v-else>
      <div
        v-for="agent in agents"
        class="avatar-list__multiple"
        :style="{ width: containerWidth }"
      >
        <!-- <template v-if="agent.avatar">
            <img
              :key="index"
              :src="agent.avatar"
              alt="avatar"
              class="avatar-list__avatar"
            />
          </template> -->
        <!-- <template v-else> -->
        <BridgeSmallIcon v-if="agent.id === BRIDGE_AGENT_ID" />
        <NoAvatarIcon
          v-else
          :key="agent.id"
          class="avatar-list__avatar"
        />
        <!-- </template> -->
      </div>
    </template>
  </div>
</template>

<style lang="scss" scoped>
  .avatar-list {
    display: flex;
    align-items: center;

    &__single,
    &__multiple {
      @include flex(row, space-between, center);
    }

    &__single {
      gap: 8px;
    }

    &__multiple {
      position: relative;

      & .avatar-list__avatar {
        position: absolute;
        right: 0;

        @for $i from 1 through 10 {
          &:nth-child(#{$i}) {
            right: 16px * ($i - 1);
            z-index: calc(10 - $i);
          }
        }
      }
    }

    &__avatar {
      object-fit: cover;
      width: 24px;
      height: 24px;
      border-radius: 50%;
    }

    &__name {
      white-space: nowrap;

      @include font-inter-500(14px, 20px, var(--text-secondary));
    }
  }
</style>
