<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { AbilitiesList } from '~/widgets/abilitiesList'
  import { useAbilitiesNavigation, useAbilitiesStore } from '~/features/ability'
  import { BaseContainer, BaseButton } from '~/shared/ui/base'
  import { LibraryIcon, PlusIcon } from '~/shared/ui/icons'
  import { ToggleSwitch } from '~/shared/ui/toggle-switch'

  definePageMeta({
    title: 'Abilities',
  })

  const entity: Ref<string> = ref('abilities')

  const { abilities } = storeToRefs(useAbilitiesStore())

  const { isCreateAbility, enableCreateAbility, selectedAbility } = useAbilitiesNavigation()

  const AbilityFullItem = defineAsyncComponent(async () => {
    const module = await import('~/widgets/abilityFullItem')
    return module.AbilityFullItem
  })

  const AbilityForm = defineAsyncComponent(async () => {
    const module = await import('~/widgets/abilityForm')
    return module.AbilityForm
  })

  const sideContentComponent = computed(() => {
    if (isCreateAbility.value) {
      return AbilityForm
    }
    if (selectedAbility.value) {
      return AbilityFullItem
    }
    return null
  })

  const createHandle = () => {
    enableCreateAbility()
  }
</script>

<template>
  <BaseContainer>
    <template #main>
      <div class="main-content">
        <div class="main-content__header">
          <ToggleSwitch
            v-model="entity"
            readonly
          >
            <template #option-agents> <div @click="() => navigateTo('/agents')">Agents</div> </template>
            <template #option-abilities>
              <div @click="() => navigateTo('/agents/abilities')">Abilities</div>
            </template>
          </ToggleSwitch>
          <BaseButton
            :disabled="isCreateAbility"
            size="medium"
            class="agents-list__create"
            @click="createHandle"
          >
            <template #icon>
              <PlusIcon />
            </template>
            Create new
          </BaseButton>
        </div>
        <div class="list-title"><LibraryIcon /> Library <span>4</span></div>
        <AbilitiesList :abilities="abilities" />
      </div>
    </template>
    <template
      v-if="sideContentComponent"
      #additional
    >
      <div class="side-content">
        <component
          :is="sideContentComponent"
          :key="String(selectedAbility)"
        />
      </div>
    </template>
  </BaseContainer>
</template>

<style lang="scss" scoped>
  div {
    color: var(--text-primary);
  }

  .main-content {
    flex: 1;
  }

  .side-content {
    width: 100%;
    height: 100%;
    border-left: 1px solid var(--border-3);
    background: var(--surface-1);
  }

  .main-content__header {
    padding: 12px 24px 0;
    border-bottom: 1px solid var(--border-3);

    @include flex(row, space-between, center);
  }

  .list-title {
    gap: 8px;
    margin: 24px 24px 0;

    span {
      @include font-inter-400(16px, 22px, var(--text-tertiary));
    }

    @include font-inter-700(16px, 22px, var(--text-secondary));
    @include flex(row, start, center);
  }

  .agents-list__create {
    margin-top: -10px;
  }
</style>
