<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { useAgentsStore } from '~/features/agent'
  import { getTask, getTaskResults, useTasksStore } from '~/features/task'
  import type { Agent } from '~/entities/agents'
  import { AgentSelector } from '~/entities/agents'
  import { TaskStatusBadge, TaskTitle, TaskSummary, TaskStatus, TaskItemLine, type Task } from '~/entities/tasks'
  import { getTimeAgo, utcToLocalTime } from '~/shared/lib'
  import { FilesList } from '~/shared/ui/files'
  import { ArrowLeftIcon, AttachmentIcon, ResultIcon } from '~/shared/ui/icons'
  import ActivityFeed from './ActivityFeed.vue'
  import TaskControls from './TaskControls.vue'
  import TaskResult from './TaskResult.vue'

  const { updateTask, selectTask } = useTasksStore()
  const { selectedTask: task, selectedTaskParentId } = storeToRefs(useTasksStore())
  const route = useRoute()
  if (!task.value) {
    const taskIdQuery = isNaN(Number(route.query.task)) ? null : Number(route.query.task)
    if (taskIdQuery) {
      await selectTask(taskIdQuery)
    } else {
      navigateTo('/tasks')
    }
  }

  const taskResults = ref(await getTaskResults(task.value!.id))

  const updateResults = async () => {
    taskResults.value = await getTaskResults(task.value!.id)
  }

  watch(
    () => task.value,
    async (newVal) => {
      if (newVal) {
        taskTitle.value = task.value!.title
        taskSummary.value = task.value!.summary
        agent.value = getAgentById(task.value!.agent_id!)!
        updateResults()
        if (selectedTaskParentId.value) {
          taskAncestor.value = await getTask(selectedTaskParentId.value)
        } else {
          taskAncestor.value = null
        }
      }
    },
    {
      deep: true,
    },
  )

  const createdAt = computed(() => {
    if (task.value) {
      const localTime = utcToLocalTime(task.value.created_at)
      return getTimeAgo({ date: localTime.toDate(), dateFormat: 'DD.MM.YYYY, HH:mm', fullUnit: true }).value
    }
    return ''
  })

  const { getById: getAgentById } = useAgentsStore()
  const { agents } = storeToRefs(useAgentsStore())

  const agent = ref<Agent>(getAgentById(task.value!.agent_id!)!)

  const taskIsEditable = computed(() => {
    return [TaskStatus.DRAFT, TaskStatus.FAILED, TaskStatus.WAITING_FOR_USER].includes(task.value!.status)
  })

  const taskTitle = ref(task.value!.title)

  const handleUpdate = async () => {
    const { id } = await updateTask({
      id: task.value!.id,
      title: taskTitle.value,
      summary: taskSummary.value,
      agent_id: agent.value.id,
    })

    return id
  }

  const taskSummary = ref(task.value!.summary)

  const taskAncestor = ref<Nullable<Task>>(null)
  if (selectedTaskParentId.value) {
    taskAncestor.value = await getTask(selectedTaskParentId.value)
  }
</script>
<template>
  <div class="task-details">
    <div class="task-details__head">
      <div class="task-details__title">
        <b>Task #{{ task!.id }}</b> {{ createdAt }}
      </div>

      <TaskControls :task="task!" />
    </div>
    <div
      v-if="taskAncestor"
      class="task-details__ancestor"
      @click="selectTask(taskAncestor.id)"
    >
      <ArrowLeftIcon />
      <div class="task-details__ancestor-title">
        {{ taskAncestor.title }}
      </div>
    </div>
    <div class="task-details__body">
      <!-- TODO: back to parent task -->
      <!-- <div class="task-details__back">
        <ArrowLeftIcon /> Define the key requirements from the client for Brand Analytics functionality.
      </div> -->
      <div class="task-details__top">
        <div class="task-details__status">
          <TaskStatusBadge :status="task!.status" />
        </div>
        <AgentSelector
          v-model="agent"
          :agents="agents"
          :disabled="!taskIsEditable"
          @update:model-value="handleUpdate"
        />
      </div>
      <div class="task-details__middle">
        <TaskTitle
          v-model="taskTitle"
          :current-title="task!.title"
          :task-id="task!.id"
          @save="handleUpdate"
        />

        <TaskSummary
          v-model="taskSummary"
          :current-summary="task!.summary"
          @save="handleUpdate"
        />

        <div class="task-details__attachments">
          <div class="task-details__attachments-title">
            <AttachmentIcon
              width="20px"
              height="20px"
            />
            Attachments
          </div>
          <div class="task-details__attachments-add">+ Add</div>
        </div>
        <FilesList :files="[]" />
      </div>
      <div
        v-if="taskResults.length"
        class="task-details__result"
      >
        <div class="task-details__result-head"><ResultIcon /> Result</div>
        <TaskResult
          v-for="result in taskResults"
          :key="result.id"
          :result="result"
        />

        <!-- <LargeFilesPreview
        :files="[
          {
            type: 'TXT',
            url: 'file.txt',
            name: 'file.txt',
            created: '14.07.2024, 18:32',
            rows: 10,
            size: '1.2 MB',
          },
        ]"
      /> -->
      </div>
      <div
        v-if="task?.children.length"
        class="task-details__children"
      >
        <TaskItemLine
          v-for="childTask in task.children"
          :key="childTask.id"
          :task="childTask"
          is-child
          @click="selectTask(childTask.id)"
        />
      </div>
    </div>

    <ActivityFeed v-if="task?.status !== TaskStatus.DRAFT" />
  </div>
</template>
<style scoped lang="scss">
  .task-details {
    overflow: hidden;
    height: 100%;

    &__ancestor {
      margin: 24px 16px 0;
      padding: 8px;
      border-radius: 6px;
      background-color: var(--surface-3);
      color: var(--text-secondary);
      text-decoration: none;

      & svg {
        flex-shrink: 0;
      }

      &:hover {
        background-color: var(--surface-4);
        color: var(--text-primary);
      }

      &-title {
        @include text-ellipsis;
      }

      @include flex($align: center, $gap: 8px);
    }

    &__head {
      height: 56px;
      padding: 0 24px;
      border-bottom: 0.5px solid var(--border-3);

      @include flex(row, space-between, center);
    }

    &__title {
      b {
        color: var(--text-secondary);
      }

      @include font-inter-400(14px, 20px, var(--text-tertiary));
      @include flex(row, start, center, 8px);
    }

    &__back {
      gap: 4px;
      padding: 8px;
      background: var(--surface-3);

      @include flex(row, flex-start, space-between);
      @include font-inter-700(12px, 22px, var(--text-tertiary));
    }

    &__body {
      flex: 1;
      overflow: hidden auto;
      height: 50%;
      padding: 24px 12px;

      @include add-scrollbar;
      @include flex(column, $gap: 8px);
    }

    &__top {
      padding: 0 12px;

      @include flex(row, space-between, center);
    }

    &__middle {
      @include flex(column, $gap: 8px);
    }

    &__text-title {
      margin-bottom: 16px;

      @include font-inter-500(16px, 22px, var(--text-primary));
    }

    &__text {
      position: relative;

      @include font-inter-400(14px, 19px, var(--text-secondary));
    }

    &__attachments {
      margin-top: 16px;
      padding: 0 12px 0 10px;

      &-title {
        @include font-inter-500(14px, 20px, var(--text-secondary));
        @include flex(row, flex-start, center, 8px);
      }

      &-add {
        @include font-inter-500(14px, 20px, var(--button-primary));
      }

      @include flex(row, space-between, center);
    }

    &__result {
      padding: 24px 12px;
    }

    &__result-head {
      gap: 2px;
      margin-bottom: 24px;

      @include font-inter-500(14px, 22px, var(--text-tertiary));
      @include flex(row, flex-start, center);
    }

    &__result-text-wrapper {
      padding: 12px;
      border-radius: 12px;
      background: var(--surface-3);
    }

    &__result-title {
      margin-bottom: 24px;

      @include font-inter-500(16px, 25px, var(--text-secondary));
    }

    &__result-text {
      @include font-inter-500(14px, 22px, var(--text-secondary));
    }

    @include flex(column, flex-start, stretch);
  }
</style>
