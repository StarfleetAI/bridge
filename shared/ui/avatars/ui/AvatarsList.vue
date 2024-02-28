<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { NoAvatarIcon } from '~/shared/ui/icons'
  import { type Person } from '../model'

  const props = defineProps<{
    persons: Person[]
  }>()

  const containerWidth = computed(() => {
    return `${(props.persons.length - 1) * 16 + 24}px`
  })
</script>

<template>
  <div class="avatar-list">
    <template v-if="persons.length === 1">
      <div class="avatar-list__single">
        <div class="avatar-list__name">{{ persons[0].name }}</div>
        <template v-if="persons[0].avatar">
          <img
            :src="persons[0].avatar"
            alt="avatar"
            class="avatar-list__avatar"
          />
        </template>
        <template v-else>
          <NoAvatarIcon class="avatar-list__avatar" />
        </template>
      </div>
    </template>
    <template v-else>
      <div
        class="avatar-list__multiple"
        :style="{ width: containerWidth }"
      >
        <template v-for="(person, index) in persons">
          <template v-if="person.avatar">
            <img
              :key="index"
              :src="person.avatar"
              alt="avatar"
              class="avatar-list__avatar"
            />
          </template>
          <template v-else>
            <NoAvatarIcon
              :key="index"
              class="avatar-list__avatar"
            />
          </template>
        </template>
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
      @include font-inter-500(14px, 20px, var(--text-secondary));
    }
  }
</style>
