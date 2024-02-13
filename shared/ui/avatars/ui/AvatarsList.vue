<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
import { defineProps } from 'vue'
import { NoAvatarIcon } from '~/shared/icons'
import { type Person } from "../model"

defineProps<{
  persons: Person[];
}>();
</script>

<template>
  <div class="avatar-list">
    <template v-if="persons.length === 1">
      <div class="avatar-list__single">
        <span class="avatar-list__name">{{ persons[0].name }}</span>
        <template v-if="persons[0].avatar">
          <img :src="persons[0].avatar" alt="avatar" class="avatar-list__avatar" />
        </template>
        <template v-else>
          <NoAvatarIcon class="avatar-list__avatar" />
        </template>
      </div>
    </template>
    <template v-else>
      <div class="avatar-list__multiple">
        <template v-for="(person, index) in persons" :key="index">
          <template v-if="person.avatar">
            <img :src="person.avatar" alt="avatar" class="avatar-list__avatar" />
          </template>
          <template v-else>
            <NoAvatarIcon class="avatar-list__avatar" />
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

  &__avatar {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    object-fit: cover;
    margin-left: -6px;
  }

  &__name {
    margin-right: 8px;

    @include font-inter-500(14px, 22px, var(--text-secondary));
  }
}
</style>
