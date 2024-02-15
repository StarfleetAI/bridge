<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { UserAvatar } from '~/entities/profile'
  import { useLastTabRoute, type TabRoute } from '~/shared/lib'
  import { AgentsIcon, ChatsIcon, DocumentsIcon, SearchIcon, SettingsIcon, TasksIcon } from '~/shared/ui/icons'
  import HeaderNavigationItem from './HeaderNavigationItem.vue'

  const { getTabLastRoute } = useLastTabRoute()

  const getTabRoute = (tab: TabRoute) => {
    const tabRoute = getTabLastRoute(tab)
    if (tabRoute) {
      return tabRoute
    }
    return { name: tab }
  }
</script>

<template>
  <div class="default-layout__header">
    <div class="default-layout__header-tabs">
      <HeaderNavigationItem
        name="chats-id"
        :to="getTabRoute('chats-id')"
      >
        <template #icon>
          <ChatsIcon />
        </template>
        <template #name> Chats </template>
      </HeaderNavigationItem>
      <HeaderNavigationItem
        name="tasks"
        :to="getTabRoute('tasks')"
      >
        <template #icon>
          <TasksIcon />
        </template>
        <template #name> Tasks </template>
      </HeaderNavigationItem>
      <HeaderNavigationItem
        name="agents"
        :to="getTabRoute('agents')"
      >
        <template #icon>
          <AgentsIcon />
        </template>
        <template #name> Agents </template>
      </HeaderNavigationItem>
      <HeaderNavigationItem
        name="documents"
        :to="getTabRoute('documents')"
      >
        <template #icon>
          <DocumentsIcon />
        </template>
        <template #name> Documents </template>
      </HeaderNavigationItem>
      <HeaderNavigationItem
        name="settings"
        :to="getTabRoute('settings')"
      >
        <template #icon>
          <SettingsIcon />
        </template>
        <template #name> Settings </template>
      </HeaderNavigationItem>
    </div>
    <div class="default-layout__header-control">
      <SearchIcon class="default-layout__header-search" />
      <div class="default-layout__header-tasks">
        {{ 5 }}
      </div>
      <UserAvatar />
    </div>
  </div>
</template>

<style scoped lang="scss">
  .default-layout__header {
    width: 100%;
    height: 44px;
    background: var(--surface-0);

    @include flex(row, space-between, center);
  }

  .default-layout__header-tabs {
    height: 100%;
    @include flex(row);
  }

  .default-layout__header-control {
    padding-right: 24px;

    @include flex(row, $align-items: center, $gap: 24px);
  }

  .default-layout__header-search {
    color: var(--text-tertiary);
  }

  .default-layout__header-tasks {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background-color: var(--status-progress);
    outline: 2px solid rgba(#00a155, 0.3);

    @include font-inter-700(12px, 17px, var(--surface-0));
    @include flex(row, center, center);
  }
</style>
