<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { TaskItem } from '~/entities/tasks'

  const view = ref('list')
</script>

<template>
  <div class="tasks-list__header">
    <div class="tasks-list__title">Active tasks</div>
    <div class="tasks-list__views" />
  </div>
  <div
    class="tasks-list"
    :class="{ 'tasks-list--list': view === 'list', 'tasks-list--grid': view === 'grid' }"
  >
    <TaskItem />
    <TaskItem />
    <TaskItem />
    <TaskItem />
  </div>
</template>
<style scoped lang="scss">
  .tasks-list {
    &__header {
      margin-bottom: 24px;

      @include flex(row, space-between, center);
    }

    &__title {
      @include font-inter-700(16px, 22px, var(--text-secondary));
    }

    &--grid {
      display: grid;
      columns: 2;
    }
  }
</style>
