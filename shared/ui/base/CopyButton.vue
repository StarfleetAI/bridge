<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  const props = defineProps<{
    content: string
  }>()

  const isCopied = ref(false)
  const handleClick = () => {
    navigator.clipboard.writeText(props.content)
    isCopied.value = true
    setTimeout(() => {
      isCopied.value = false
    }, 3000)
  }
  const copyText = computed(() => (isCopied.value ? 'Copied!' : 'Copy'))
</script>

<template>
  <div
    class="copy-button"
    @click="handleClick"
  >
    {{ copyText }}
  </div>
</template>

<style lang="scss" scoped>
  .copy-button {
    position: relative;
    padding-left: 24px;
    text-align: right;

    &:before {
      content: '';
      position: absolute;
      top: 2px;
      left: 0;
      width: 16px;
      height: 16px;
      background: url('~/assets/svg/copy-icon.svg') no-repeat left;
    }

    @include font-inter-500(14px, 20px, var(--text-secondary));
  }
</style>
