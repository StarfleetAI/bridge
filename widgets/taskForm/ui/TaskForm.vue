<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAgentsStore } from '~/features/agent'
  import { useTasksStore } from '~/features/task'
  import { AgentSelector } from '~/entities/agents'
  import { TaskStatus, TaskStatusBadge, TaskSummary, TaskTitle } from '~/entities/tasks'
  import { BaseButton } from '~/shared/ui/base'
  import { FilesList } from '~/shared/ui/files'
  import { AttachmentIcon, CrossIcon, SaveIcon, StarsIcon } from '~/shared/ui/icons'
  const { agents } = storeToRefs(useAgentsStore())
  const { listRootTasksByStatus, createTask, selectTask, setIsNewTask } = useTasksStore()
  const selectedAgent = ref(agents.value[0])

  const taskTitle = ref('')

  const taskSummary = ref('')
  const saveIsEnabled = computed(() => taskTitle.value.length > 0)

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

  const handleExecuteTask = async (status: TaskStatus) => {
    const newTask = await createTask(getTaskEntity(status))
    listRootTasksByStatus()
    selectTask(newTask.id)
  }

  const handleRemoveFile = (file: File) => {
    selectedFiles.value = selectedFiles.value.filter((selectedFile) => selectedFile.name !== file.name)
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
          @click="handleExecuteTask(TaskStatus.DRAFT)"
        >
          <template #icon>
            <SaveIcon />
          </template>
          Save
        </BaseButton>
        <BaseButton
          :disabled="!saveIsEnabled"
          @click="handleExecuteTask(TaskStatus.TODO)"
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
          @click="setIsNewTask(false)"
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
        <TaskTitle
          v-model="taskTitle"
          placeholder="Task Title"
        />
        <TaskSummary
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
          Attachments
        </div>
        <div
          class="task-form__files-add"
          @click="openFileDialog()"
        >
          + Add
        </div>
      </div>
      <FilesList
        v-if="selectedFiles.length"
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
    height: 56px;
    padding: 12px 24px;
    border-bottom: 0.5px solid var(--border-3);

    @include flex(row, space-between, center);
  }

  .task-form__title {
    @include font-inter-500(14px, 20px, var(--text-secondary));
  }

  .task-form__actions {
    @include flex(row, flex-end, center, 16px);
  }

  .task-form__body {
    padding: 24px 12px;

    // border-bottom: 1px solid var(--border-3);

    @include flex(column, $gap: 8px);
  }

  .task-form__body-top {
    padding: 0 12px;

    @include flex(row, space-between, center);
  }

  .task-form__input-container {
    @include flex(column, flex-start, stretch, 8px);
  }

  .task-form__input-description {
    padding: 8px 12px;

    @include font-inter-400(14px, 20px, var(--text-tertiary));
  }

  .task-form__files-container {
    margin-top: 16px;
    padding: 0 12px 0 10px;

    @include flex(row, space-between, center);
  }

  .task-form__files-list {
    @include font-inter-500(14px, 20px, var(--text-secondary));
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
