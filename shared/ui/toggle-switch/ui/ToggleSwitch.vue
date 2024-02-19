<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

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

<script lang="ts" setup>
  interface Props {
    modelValue: string
  }

  const { modelValue } = defineProps<Props>()
  const emit = defineEmits(['update:modelValue'])
  const slots = useSlots()
  const slotElements = computed(() => Object.keys(slots).filter((name) => name.startsWith('option-')))

  const setActive = (value: string) => {
    emit('update:modelValue', value.replace('option-', ''))
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

<style scoped lang="scss">
  .toggle-switch {
    display: flex;
    border-radius: 6px;
    background: var(--side-panel);
    cursor: pointer;

    & > div {
      gap: 8px;
      padding: 8px 16px;
      border-radius: 6px;
      transition: background-color 0.3s ease;

      &.active {
        background: var(--surface-5);
        color: var(--text-primary);

        svg {
          stroke: var(--text-primary) !important;
        }
      }

      @include flex(row, center, center);
      @include font-inter-500(14px, 19px, var(--text-tertiary));
    }
  }
</style>
