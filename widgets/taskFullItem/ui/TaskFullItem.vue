<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script setup lang="ts">
  import { useAgentsStore } from '~/features/agent'
  import { useTasksNavigation, useTasksStore } from '~/features/task'
  import { TaskStatusBadge, type Task, TaskStatus, TaskInput } from '~/entities/tasks'
  import { getTimeAgo, utcToLocalTime } from '~/shared/lib'
  import { AvatarsList } from '~/shared/ui/avatars'
  import { FilesList, LargeFilesPreview } from '~/shared/ui/files'
  import { ResultIcon, AttachmentIcon } from '~/shared/ui/icons'
  import TaskControls from './TaskControls.vue'

  const { selectedTask } = useTasksNavigation()

  const { getById, updateTask } = useTasksStore()
  if (!getById(selectedTask.value!)) {
    navigateTo('/tasks')
  }
  const task = ref(getById(selectedTask.value!) as Task)

  const setUpdatedTask = (updatedTask: Task) => {
    task.value = updatedTask
  }

  const createdAt = computed(() => {
    if (task.value) {
      const localTime = utcToLocalTime(task.value.created_at)
      return getTimeAgo({ date: localTime.toDate(), dateFormat: 'DD.MM.YYYY, HH:mm', fullUnit: true }).value
    }
    return ''
  })

  const { getById: getAgentById } = useAgentsStore()

  const agent = computed(() => {
    return getAgentById(task.value.agent_id)
  })

  const taskIsEditable = computed(() => {
    return [
      TaskStatus.DRAFT,
      TaskStatus.PAUSED,
      TaskStatus.CANCELED,
      TaskStatus.FAILED,
      TaskStatus.WAITING_FOR_USER,
    ].includes(task.value.status)
  })

  const taskTitle = ref(task.value.title)
  const taskTitlePlaceholder = computed(() => {
    return task.value.title || `Task #${task.value.id}`
  })
  const titleIsEditing = ref(false)
  const titleInput = ref<InstanceType<typeof TaskInput> | null>(null)
  const enableTitleEditing = () => {
    if (!taskIsEditable.value || titleIsEditing.value) {
      return
    }
    titleIsEditing.value = true
    nextTick(() => {
      titleInput.value?.focus()
    })
  }

  const handleUpdate = async () => {
    const { id } = await updateTask({ id: task.value.id, title: taskTitle.value, summary: taskSummary.value })
    setUpdatedTask(getById(id)!)
    return id
  }

  const updateTitle = async () => {
    await handleUpdate()
    titleIsEditing.value = false
  }
  const titleComponent = computed(() => {
    if (titleIsEditing.value) {
      return TaskInput
    }
    return 'div'
  })

  const taskSummary = ref(task.value.summary)
  const summaryIsEditing = ref(false)
  const summaryInput = ref<InstanceType<typeof TaskInput> | null>(null)
  const enableSummaryEditing = () => {
    if (!taskIsEditable.value || summaryIsEditing.value) {
      return
    }
    summaryIsEditing.value = true
    nextTick(() => {
      nextTick(() => {
        summaryInput.value?.focus()
      })
    })
  }

  const updateSummary = async () => {
    await handleUpdate()
    summaryIsEditing.value = false
  }
  const summaryComponent = computed(() => {
    if (summaryIsEditing.value) {
      return TaskInput
    }
    return 'div'
  })
</script>
<template>
  <div class="task-details">
    <div class="task-details__head">
      <div class="task-details__title">
        <b>Task #{{ task.id }}</b> {{ createdAt }}
      </div>

      <TaskControls
        :task="task"
        @update="setUpdatedTask"
      />
    </div>
    <div class="task-details__body">
      <!-- TODO: back to parent task -->
      <!-- <div class="task-details__back">
        <ArrowLeftIcon /> Define the key requirements from the client for Brand Analytics functionality.
      </div> -->
      <div class="task-details__top">
        <div class="task-details__status">
          <TaskStatusBadge :status="task.status" />
        </div>
        <AvatarsList
          v-if="agent"
          :persons="[{ name: agent?.name, avatar: '', link: '' }]"
        />
      </div>
      <div class="task-details__middle">
        <component
          :is="titleComponent"
          ref="titleInput"
          v-model="taskTitle"
          :class="[
            'task__title',
            {
              'task__title--editing': titleIsEditing,
              'task__title--editable': taskIsEditable,
            },
          ]"
          @click="enableTitleEditing"
          @blur="updateTitle"
        >
          {{ taskTitlePlaceholder }}
        </component>

        <component
          :is="summaryComponent"
          ref="summaryInput"
          v-model="taskSummary"
          :class="[
            'task__description',
            {
              'task__description--editing': summaryIsEditing,
              'task__description--editable': taskIsEditable,
            },
          ]"
          @blur="updateSummary"
          @click="enableSummaryEditing"
        >
          {{ task.summary || 'No summary' }}
        </component>

        <div class="task-details__attachments">
          <div class="task-details__attachments-title">
            <AttachmentIcon
              width="20"
              height="20"
            />
            Attachments
          </div>
          <div>+ Add</div>
        </div>
        <FilesList :files="[]" />
      </div>
    </div>
    <div class="task-details__result">
      <div class="task-details__result-head"><ResultIcon /> Result</div>
      <LargeFilesPreview
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
      />
      <div class="task-details__result-text-wrapper">
        <div class="task-details__result-title">Stage 1</div>
        <div class="task-details__result-text">
          This task involves identifying and implementing robust methods to guarantee the security of data at every
          stage of Brand Analytics, from collection through storage to analysis. It is crucial to establish a
          comprehensive approach that safeguards sensitive information and maintains the integrity and confidentiality
          of the data processed by the neural network.
        </div>
      </div>
    </div>
  </div>
</template>
<style scoped lang="scss">
  .task-details {
    &__head {
      height: 57px;
      padding: 12px 24px;
      border-bottom: 1px solid var(--border-3);

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
      cursor: pointer;

      @include flex(row, flex-start, space-between);
      @include font-inter-700(12px, 22px, var(--text-tertiary));
    }

    &__body {
      padding: 24px 0;
      border-bottom: 0.5px solid var(--pill);
    }

    &__top {
      padding: 0 24px;

      @include flex(row, space-between, space-between);
    }

    &__middle {
      margin-top: 8px;
    }

    &__text-title {
      margin-bottom: 16px;

      @include font-inter-500(16px, 22px, var(--text-primary));
    }

    &__text {
      position: relative;

      // &:after {
      //   content: '';
      //   position: absolute;
      //   right: 0;
      //   bottom: 0;
      //   left: 0;
      //   height: 50px;
      //   background: linear-gradient(to bottom, rgba(36, 39, 49, 0), rgba(36, 39, 49, 1));
      // }

      @include font-inter-400(14px, 19px, var(--text-secondary));
    }

    &__attachments {
      margin-top: 16px;
      padding: 0 24px;

      &-title {
        color: var(--text-secondary);

        @include flex(row, flex-start, center, 8px);
      }

      @include font-inter-500(14px, 22px, var(--text-tertiary));
      @include flex(row, space-between, center);
    }

    &__result {
      padding: 24px;
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
  }

  .task__title {
    @include font-inter-500(18px, 25px, var(--text-primary));
  }

  .task__description {
    @include font-inter-400(14px, 20px, var(--text-secondary));
  }

  .task__title,
  .task__description {
    margin: 0 12px;
    padding: 8px 12px;
    border-radius: 6px;

    &--editing {
      width: calc(100% - 24px);
      margin-bottom: -6px;
    }

    &--editable:hover {
      background-color: var(--surface-3);
    }
  }
</style>
