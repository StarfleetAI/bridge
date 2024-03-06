<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { useAbilitiesStore } from '~/features/ability'
  import { type Ability } from '~/entities/abilities'
  import { PlusIcon, DeleteIcon } from '~/shared/ui/icons'
  const props = defineProps<{
    modelValue: Ability[]
  }>()
  // eslint-disable-next-line sonarjs/no-duplicate-string
  const emit = defineEmits(['update:modelValue'])

  const { abilities } = storeToRefs(useAbilitiesStore())

  const selectedAbilities: Ref<Ability[]> = ref([])

  onMounted(() => {
    selectedAbilities.value = props.modelValue.length > 0 ? props.modelValue : []
  })

  const addAbility = (ability: Ability) => {
    selectedAbilities.value.push(ability)
    emit('update:modelValue', selectedAbilities.value)
  }
  const removeAbility = (ability: Ability) => {
    const index = selectedAbilities.value.findIndex((a) => a.id === ability.id)
    if (index !== -1) {
      selectedAbilities.value.splice(index, 1)
      emit('update:modelValue', selectedAbilities.value)
    }
  }

  const toggleAbility = (ability: Ability) => {
    if (selectedAbilities.value.some((a) => a.id === ability.id)) {
      removeAbility(ability)
    } else {
      addAbility(ability)
    }
  }

  const isAbilitySelected = (ability: Ability) => {
    return selectedAbilities.value.some((a) => a.id === ability.id)
  }
</script>

<template>
  <div class="abilities-select-list__title">Add Abilities</div>
  <div class="abilities-select-list">
    <div
      v-for="ability in abilities"
      :key="ability.id"
      class="abilities-select-list__item"
      @click="toggleAbility(ability)"
    >
      <div class="abilities-select-list__item-control">
        <template v-if="isAbilitySelected(ability)">
          <DeleteIcon />
        </template>
        <template v-else>
          <PlusIcon />
        </template>
      </div>
      <div class="abilities-select-list__item-info">
        <div class="abilities-select-list__item-name">{{ ability.name }}</div>
        <div class="abilities-select-list__item-description">{{ ability.description }}</div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
  .abilities-select-list {
    min-width: 400px;

    &__title {
      margin-bottom: 18px;

      @include font-inter-700(14px, 20px, var(--text-secondary));
    }

    &__item {
      width: 100%;
      height: 36px;
      margin-bottom: 6px;
      border-radius: 6px;
      background: var(--surface-2);

      @include flex(row, start, start);
    }

    &__item-control {
      width: 36px;
      height: 36px;
      border-top-left-radius: 6px;
      border-bottom-left-radius: 6px;
      background: var(--surface-3);
      cursor: pointer;

      @include flex(row, center, center);
    }

    &__item-info {
      padding: 8px 16px;

      @include flex(row, start, center);
    }

    &__item-name {
      margin-right: 8px;

      @include font-inter-500(14px, 20px, var(--text-secondary));
    }

    &__item-description {
      overflow: hidden;
      max-width: 200px;
      text-overflow: ellipsis;
      white-space: nowrap;

      @include font-inter-400(12px, 21px, var(--text-tertiary));
    }
  }
</style>
