<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { UserAvatar } from '~/entities/profile'
  import { useLastTabRoute, type TabRoute } from '~/shared/lib'
  import { AgentsIcon, ChatsIcon, ClipboardIcon, DocumentsIcon, SearchIcon, SettingsIcon } from '~/shared/ui/icons'
  import SidebarNavigationItem from './SidebarNavigationItem.vue'

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
      <SidebarNavigationItem
        name="tasks"
        :to="getTabRoute('tasks')"
      >
        <template #icon>
          <ClipboardIcon
            width="24"
            height="24"
          />
        </template>
        <template #name> Tasks </template>
      </SidebarNavigationItem>
      <SidebarNavigationItem
        name="chats"
        :to="getTabRoute('chats')"
      >
        <template #icon>
          <ChatsIcon
            width="24"
            height="24"
          />
        </template>
        <template #name> Chats </template>
      </SidebarNavigationItem>
      <SidebarNavigationItem
        name="agents"
        :to="getTabRoute('agents')"
      >
        <template #icon>
          <AgentsIcon
            width="24"
            height="24"
          />
        </template>
        <template #name> Agents </template>
      </SidebarNavigationItem>
      <SidebarNavigationItem
        name="documents"
        :to="getTabRoute('documents')"
      >
        <template #icon>
          <DocumentsIcon
            width="24"
            height="24"
          />
        </template>
        <template #name> Docs </template>
      </SidebarNavigationItem>
      <SidebarNavigationItem
        name="settings"
        :to="getTabRoute('settings')"
      >
        <template #icon>
          <SettingsIcon
            width="24"
            height="24"
          />
        </template>
        <template #name> Settings </template>
      </SidebarNavigationItem>
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
    width: 64px;
    height: 100svh;
    background: var(--surface-0);

    @include flex(column, space-between, center);
  }

  .default-layout__header-tabs {
    width: 100%;
    height: 100svh;

    @include flex(column, $align-items: center);
  }

  .default-layout__header-control {
    padding-bottom: 24px;

    @include flex(column, $align-items: center, $gap: 24px);
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
