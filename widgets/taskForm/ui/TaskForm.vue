<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useTasksNavigation } from '~/features/task'
  import { CrossIcon } from '~/shared/ui/icons'

  const { disableCreateTask } = useTasksNavigation()
</script>

<template>
  <div class="task-form">
    <div class="task-form__header">
      <div class="task-form__title">Create Task</div>
      <div class="task-form__actions">
        <div class="task-form__action">Save</div>
        <div class="task-form__action">Execute</div>
        <div
          class="task-form__action"
          @click="disableCreateTask"
        >
          <CrossIcon />
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped></style>
