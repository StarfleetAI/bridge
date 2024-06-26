// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export { default as AbilitiesIcon } from './AbilitiesIcon.vue'
export { default as AbilityIcon } from './AbilityIcon.vue'
export { default as AgentChatIcon } from './AgentChatIcon.vue'
export { default as AgentIcon } from './AgentIcon.vue'
export { default as AgentsIcon } from './AgentsIcon.vue'
export { default as ArrowLeftIcon } from './ArrowLeftIcon.vue'
export { default as AttachmentIcon } from './AttachmentIcon.vue'
export { default as BridgeLargeIcon } from './BridgeLargeIcon.vue'
export { default as BridgeSmallIcon } from './BridgeSmallIcon.vue'
export { default as CancelIcon } from './CancelIcon.vue'
export { default as ChatsIcon } from './ChatsIcon.vue'
export { default as CheckIcon } from './CheckIcon.vue'
export { default as ChevronDownIcon } from './ChevronDownIcon.vue'
export { default as ClipboardIcon } from './ClipboardIcon.vue'
export { default as CloudIcon } from './CloudIcon.vue'
export { default as CodeIcon } from './CodeIcon.vue'
export { default as CommentsIcon } from './CommentsIcon.vue'
export { default as CopyIcon } from './CopyIcon.vue'
export { default as CrossIcon } from './CrossIcon.vue'
export { default as CubeIcon } from './CubeIcon.vue'
export { default as DeleteIcon } from './DeleteIcon.vue'
export { default as DislikeIcon } from './DislikeIcon.vue'
export { default as DocumentCheck } from './DocumentCheck.vue'
export { default as DocumentIcon } from './DocumentIcon.vue'
export { default as DocumentPreview } from './DocumentPreview.vue'
export { default as DocumentsIcon } from './DocumentsIcon.vue'
export { default as DocumentTitleIcon } from './DocumentTitleIcon.vue'
export { default as DownloadIcon } from './DownloadIcon.vue'
export { default as DropdownIcon } from './DropdownIcon.vue'
export { default as DuplicateIcon } from './DuplicateIcon.vue'
export { default as EditIcon } from './EditIcon.vue'
export { default as FileCSVIcon } from './FileCSVIcon.vue'
export { default as FileFrameIcon } from './FileFrameIcon.vue'
export { default as FileIcon } from './FileIcon.vue'
export { default as FileTXTIcon } from './FileTXTIcon.vue'
export { default as KebabIcon } from './KebabIcon.vue'
export { default as LibraryIcon } from './LibraryIcon.vue'
export { default as LogIcon } from './LogIcon.vue'
export { default as NoAvatarIcon } from './NoAvatarIcon.vue'
export { default as PauseIcon } from './PauseIcon.vue'
export { default as PinIcon } from './PinIcon.vue'
export { default as PlusIcon } from './PlusIcon.vue'
export { default as ResizeIcon } from './ResizeIcon.vue'
export { default as ResultIcon } from './ResultIcon.vue'
export { default as RetryIcon } from './RetryIcon.vue'
export { default as ReviseIcon } from './ReviseIcon.vue'
export { default as SaveIcon } from './SaveIcon.vue'
export { default as SearchIcon } from './SearchIcon.vue'
export { default as SendIcon } from './SendIcon.vue'
export { default as SettingsIcon } from './SettingsIcon.vue'
export { default as StoreIcon } from './StoreIcon.vue'
export { default as SystemIcon } from './SystemIcon.vue'
export { default as TaskHeaderIcon } from './TaskHeaderIcon.vue'
export { default as TasksIcon } from './TasksIcon.vue'
export { default as UnlinkIcon } from './UnlinkIcon.vue'
export { default as UnpinIcon } from './UnpinIcon.vue'

// Async components
const ChevronLeftIcon = () => import('./ChevronLeftIcon.vue')
const ChevronRightIcon = () => import('./ChevronRightIcon.vue')
const PenIcon = () => import('./PenIcon.vue')
const ResumeIcon = () => import('./ResumeIcon.vue')
const StarsIcon = () => import('./StarsIcon.vue')
const TaskStatusDone = () => import('./TaskStatusDone.vue')
const TaskStatusDraft = () => import('./TaskStatusDraft.vue')
const TaskStatusFailed = () => import('./TaskStatusFailed.vue')
const TaskStatusInProgress = () => import('./TaskStatusInProgress.vue')
const TaskStatusToDo = () => import('./TaskStatusToDo.vue')
const TaskStatusWaiting = () => import('./TaskStatusWaiting.vue')
export {
  ChevronLeftIcon,
  ChevronRightIcon,
  PenIcon,
  ResumeIcon,
  StarsIcon,
  TaskStatusDone,
  TaskStatusDraft,
  TaskStatusFailed,
  TaskStatusInProgress,
  TaskStatusToDo,
  TaskStatusWaiting,
}
