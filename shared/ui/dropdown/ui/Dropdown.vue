<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup generic="T">
  import type { DropdownOption } from '../model'

  const props = defineProps<{
    multiple?: boolean
    options: DropdownOption[]
  }>()

  const model = defineModel<DropdownOption[]>('modelValue', {
    default: () => [] as DropdownOption[],
  })

  const isOpen = ref(false)
  const dropdown = ref<HTMLElement | null>(null)

  const selected = ref([] as DropdownOption[])

  const initSelectedValues = () => {
    selected.value = model.value !== undefined ? model.value : []
  }

  const toggleDropdown = () => {
    isOpen.value = !isOpen.value
  }

  const closeDropdown = () => {
    isOpen.value = false
  }

  const isSelected = (label: number | string) => {
    return Boolean(selected.value.find((item) => item.label === label))
  }

  const handleSelect = (label: number | string) => {
    const fullOption = props.options.find((option) => option.label === label) as DropdownOption
    if (isSelected(label)) {
      selected.value = selected.value.filter((v) => v.label !== fullOption.label)
    } else if (props.multiple) {
      selected.value.push(fullOption)
    } else {
      selected.value = [fullOption]
      closeDropdown()
    }
    model.value = selected.value
  }

  const itemClicked = (label: number | string) => {
    if (props.multiple) {
      handleSelect(label)
    } else {
      select(label)
    }
  }

  const select = (label: number | string) => {
    const fullOption = props.options.find((option) => option.label === label) as DropdownOption
    selected.value = [fullOption]
    closeDropdown()
    model.value = selected.value
  }

  const selectedValue = computed(() => {
    return selected.value.map((val) => props.options.find((option) => option.label === val.label)?.label).join(', ')
  })

  onClickOutside(dropdown, closeDropdown)

  onMounted(() => {
    initSelectedValues()
  })

  watchEffect(() => {
    initSelectedValues()
  })
</script>

<template>
  <div
    ref="dropdown"
    class="dropdown"
  >
    <template v-if="$slots.trigger">
      <slot
        name="trigger"
        :toggleDropdown="toggleDropdown"
      />
    </template>
    <button
      v-else
      @click="toggleDropdown"
    >
      {{ selectedValue || 'Выберите опцию' }}
    </button>
    <div
      v-if="isOpen"
      :class="['dropdown-content', { 'custom-trigger': $slots.trigger }]"
    >
      <div
        v-for="option in options"
        :key="option.label"
        class="dropdown-item"
        @click="itemClicked(option.label)"
      >
        <input
          v-if="multiple"
          type="checkbox"
          :checked="isSelected(option.label)"
          @change="handleSelect(option.label)"
          @click.stop
        />
        <span>{{ option.label }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
  .dropdown {
    position: relative;
    display: inline-block;
  }

  .dropdown-content {
    position: absolute;
    z-index: 3;
    min-width: 160px;
    border-radius: 8px;
    background-color: #fff;
    box-shadow:
      0 4px 6px -2px rgba(0, 0, 0, 0.05),
      0 10px 15px -3px rgba(0, 0, 0, 0.1);

    &.custom-trigger {
      top: calc(100% + 2px);
      left: 0;
      width: 100%;
    }
  }

  .dropdown-item {
    display: flex;
    align-items: center;
    padding: 6px;
    cursor: pointer;
  }

  .dropdown-item span {
    margin-left: 8px;
  }

  .dropdown-item:hover {
    background-color: #f1f1f1;
  }
</style>
