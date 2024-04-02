<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { ResizableContainer } from '~/shared/ui/base'
  import { LogIcon } from '~/shared/ui/icons'
  import TaskComments from './TaskComments.vue'
  import TaskLogs from './TaskLogs.vue'
  type ActiveTab = 'comments' | 'logs'
  const activeTab = ref<ActiveTab>('logs')
  const setActiveTab = (tab: ActiveTab) => (activeTab.value = tab)
  const isCommentsActive = computed(() => activeTab.value === 'comments')
  const isLogsActive = computed(() => activeTab.value === 'logs')
</script>

<template>
  <ResizableContainer
    direction="top"
    max-height="80%"
    min-height="216px"
    initial-height="30%"
    class="activity-feed__container"
  >
    <div class="activity-feed">
      <div class="activity-feed__tabs-switcher">
        <!-- <div
        :class="['switcher-item', { active: isCommentsActive }]"
        @click="setActiveTab('comments')"
      >
        <CommentsIcon />
        <div class="switcher-item__label">Comments</div>
      </div> -->
        <div
          :class="['switcher-item', { active: isLogsActive }]"
          @click="setActiveTab('logs')"
        >
          <LogIcon />
          <div class="switcher-item__label">Logs</div>
        </div>
      </div>
      <div class="activity-feed__tabs">
        <TaskComments v-show="isCommentsActive" />
        <TaskLogs v-show="isLogsActive" />
      </div>
    </div>
  </ResizableContainer>
</template>

<style lang="scss" scoped>
  .activity-feed__container {
    margin-top: auto;
  }

  .activity-feed {
    height: 100%;
    border-top: 1px solid var(--border-2);
    background-color: var(--surface-3);
    box-shadow: 0 -1px 8px rgba(0, 0, 0, 0.5);

    @include flex(column);
  }

  .activity-feed__tabs-switcher {
    border-bottom: 0.5px solid var(--border-3);

    @include flex(row);
  }

  .switcher-item {
    padding: 12px 24px;

    &.active {
      color: var(--text-secondary);
    }

    @include font-inter-500(14px, 20px, var(--text-tertiary));
    @include flex(row, $align: center, $gap: 8px);
  }

  .activity-feed__tabs {
    display: flex;
    height: calc(100% - 45px);
  }
</style>
