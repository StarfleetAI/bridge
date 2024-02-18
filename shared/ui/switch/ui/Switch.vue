<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { defineProps, defineEmits } from 'vue'

  const props = defineProps<{
    modelValue: boolean
  }>()

  const emits = defineEmits<{
    (event: 'update:modelValue', value: boolean): void
  }>()

  function toggle() {
    emits('update:modelValue', !props.modelValue)
  }
</script>

<template>
  <div
    class="switch"
    :class="{ on: modelValue }"
    @click="toggle"
  >
    <div class="switch-circle" />
  </div>
</template>

<style scoped>
  .switch {
    position: relative;
    width: 30px;
    height: 16px;
    border-radius: 15px;
    background-color: var(--surface-4);
    cursor: pointer;
    transition: background-color 0.2s;

    &.on {
      background-color: var(--button-primary);

      .switch-circle {
        left: 16px;
      }
    }

    .switch-circle {
      position: absolute;
      top: 0;
      left: 0;
      width: 16px;
      height: 16px;
      border-radius: 50%;
      background-color: var(--text-primary);
      transition: left 0.2s;
    }
  }
</style>
