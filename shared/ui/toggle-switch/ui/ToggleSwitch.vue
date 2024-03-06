<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  const { modelValue, readonly } = defineProps<{
    modelValue: string
    readonly?: boolean
  }>()
  const emit = defineEmits(['update:modelValue'])
  const slots = useSlots()
  const slotElements = computed(() => Object.keys(slots).filter((name) => name.startsWith('option-')))

  const setActive = (value: string) => {
    if (!readonly) {
      emit('update:modelValue', value.replace('option-', ''))
    }
  }

  const clearTitle = (value: string) => value.replace('option-', '')

  watchEffect(() => {
    if (modelValue && slotElements.value.includes(modelValue)) {
      setActive(modelValue)
    } else if (slotElements.value.length > 0) {
      setActive(slotElements.value[0])
    }
  })
</script>

<template>
  <div class="toggle-switch">
    <div
      v-for="(slotName, index) in slotElements"
      :key="index"
      :class="{ active: modelValue === clearTitle(slotName) }"
      @click="setActive(slotName)"
    >
      <slot :name="slotName" />
    </div>
  </div>
</template>

<style scoped lang="scss">
  .toggle-switch {
    display: flex;
    border-radius: 6px;
    cursor: pointer;

    & > div {
      gap: 8px;
      padding: 8px 8px 16px;
      border-bottom: 1px solid transparent;

      &:hover {
        color: var(--text-secondary);
      }

      &.active {
        border-bottom: 1px solid var(--text-primary);
        color: var(--text-primary);

        svg {
          stroke: var(--text-primary) !important;
        }
      }

      @include flex(row, center, center);
      @include font-inter-700(14px, 19px, var(--text-tertiary));
    }
  }
</style>
