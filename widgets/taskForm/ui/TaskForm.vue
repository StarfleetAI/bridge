<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAgentsStore } from '~/features/agent'
  import { useTasksNavigation, useTasksStore } from '~/features/task'
  import { AgentSelector } from '~/entities/agents'
  import { TaskInput, TaskStatus, TaskStatusBadge } from '~/entities/tasks'
  import { BaseButton } from '~/shared/ui/base'
  import { FilesList } from '~/shared/ui/files'
  import { AttachmentIcon, CrossIcon, SaveIcon, StarsIcon } from '~/shared/ui/icons'
  const { disableCreateTask, setSelectedTask } = useTasksNavigation()
  const { agents } = storeToRefs(useAgentsStore())
  const { listRootTasks, createTask } = useTasksStore()
  const selectedAgent = ref(agents.value[0])

  const taskTitle = ref('')

  const taskSummary = ref('')
  const saveIsEnabled = computed(() => taskSummary.value.length > 0)

  const { open: openFileDialog, onChange: onFileChange } = useFileDialog()
  const selectedFiles = ref<File[]>([])

  onFileChange((newFiles) => {
    if (!newFiles) {
      return
    }
    for (let i = 0; i < newFiles.length; i++) {
      if (newFiles.item(i) !== null && !selectedFiles.value.find((file) => file.name === newFiles?.item(i)?.name)) {
        selectedFiles.value.push(newFiles.item(i)!)
      }
    }
  })
  const getTaskEntity = (status: TaskStatus) => {
    return {
      title: taskTitle.value,
      summary: taskSummary.value,
      agent_id: selectedAgent.value.id,
      status,
    }
  }
  const handleSaveTask = async () => {
    await createTask(getTaskEntity(TaskStatus.DRAFT))
    finishCreation()
  }
  const handleExecuteTask = async () => {
    const newTask = await createTask(getTaskEntity(TaskStatus.TODO))
    finishCreation()
    setSelectedTask(newTask.id)
  }

  const finishCreation = () => {
    listRootTasks({
      pagination: {
        page: 1,
        per_page: 14,
      },
    })
    disableCreateTask()
  }

  const handleRemoveFile = (file: File) => {
    selectedFiles.value = selectedFiles.value.filter((f) => f.name !== file.name)
  }
</script>

<template>
  <div class="task-form">
    <div class="task-form__header">
      <div class="task-form__title">Create Task</div>
      <div class="task-form__actions">
        <BaseButton
          color="blue"
          :disabled="!saveIsEnabled"
          @click="handleSaveTask"
        >
          <template #icon>
            <SaveIcon />
          </template>
          Save
        </BaseButton>
        <BaseButton
          :disabled="!saveIsEnabled"
          @click="handleExecuteTask"
        >
          <template #icon>
            <StarsIcon />
          </template>
          Execute
        </BaseButton>

        <CrossIcon
          color="#677383"
          height="20px"
          width="20px"
          @click="disableCreateTask"
        />
      </div>
    </div>
    <div class="task-form__body">
      <div class="task-form__body-top">
        <TaskStatusBadge :status="TaskStatus.DRAFT" />
        <AgentSelector
          v-model="selectedAgent"
          :agents="agents"
        />
      </div>
      <div class="task-form__input-container">
        <TaskInput
          v-model="taskTitle"
          placeholder="Task Title"
        />
        <TaskInput
          v-model="taskSummary"
          placeholder="Summary"
        />
      </div>
      <div class="task-form__files-container">
        <div class="task-form__files-list">
          <AttachmentIcon
            width="20px"
            height="20px"
          />
          Documents
        </div>
        <div
          class="task-form__files-add"
          @click="openFileDialog()"
        >
          +Add
        </div>
      </div>
      <FilesList
        :files="selectedFiles"
        @remove="handleRemoveFile"
      />
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .task-form {
    @include flex(column);
  }

  .task-form__header {
    padding: 12px 24px;
    border-bottom: 1px solid var(--border-3);

    @include flex(row, space-between, center);
  }

  .task-form__title {
    @include font-inter-700(14px, 20px, var(--text-secondary));
  }

  .task-form__actions {
    @include flex(row, flex-end, center, 16px);
  }

  .task-form__body {
    padding: 26px 12px;
    border-bottom: 1px solid var(--border-3);

    @include flex(column, $gap: 8px);
  }

  .task-form__body-top {
    padding: 0 12px;

    @include flex(row, space-between, center);
  }

  .task-form__input-container {
    margin-top: 8px;

    @include flex(column, flex-start, stretch, 8px);
  }

  .task-form__input {
    min-height: 41px;
    max-height: 66px;
    padding: 8px 12px;
    border: 1px solid transparent;
    border-radius: 6px;
    background-color: transparent;
    outline: none;
    resize: none;

    &.summary {
      min-height: 20px;
      max-height: 136px;

      @include font-inter-500(14px, 20px, var(--text-primary));
    }

    &:focus {
      background-color: var(--surface-3);
    }

    @include hide-scrollbar;
    @include font-inter-500(18px, 25px, var(--text-primary));
  }

  .task-form__input-description {
    padding: 8px 12px;

    @include font-inter-400(14px, 20px, var(--text-tertiary));
  }

  .task-form__files-container {
    padding: 0 12px;

    @include flex(row, space-between, center);
  }

  .task-form__files-list {
    @include font-inter-500(14px, 20px, var(--text-tertiary));
    @include flex(row, flex-start, center, 8px);
  }

  .task-form__files-previews {
    flex-wrap: wrap;
    gap: 8px;
    max-width: 60%;

    @include flex(column, flex-start, flex-start);
  }

  .task-form__files-add {
    @include font-inter-500(14px, 20px, var(--button-primary));
  }
</style>
