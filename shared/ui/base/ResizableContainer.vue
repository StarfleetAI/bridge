<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  const props = withDefaults(
    defineProps<{
      /** Value with unit (eg. px, em, rem, %, etc.)  */
      minWidth?: string
      /** Value with unit (eg. px, em, rem, %, etc.)  */
      maxWidth?: string
      /** Value with unit (eg. px, em, rem, %, etc.)  */
      minHeight?: string
      /** Value with unit (eg. px, em, rem, %, etc.)  */
      maxHeight?: string
      /** Value with unit (eg. px, em, rem, %, etc.)  */
      initialHeight?: string
      /** Value with unit (eg. px, em, rem, %, etc.)  */
      initialWidth?: string
      /** Resizer position */
      direction: 'left' | 'right' | 'top'
    }>(),
    {
      minWidth: 'initial',
      maxWidth: 'initial',
      minHeight: 'initial',
      maxHeight: 'initial',
      initialHeight: 'initial',
      initialWidth: 'initial',
    },
  )
  const container = ref<HTMLDivElement>()
  const resizer = ref<HTMLDivElement>()

  const resizeHandler = (event: MouseEvent) => {
    event.preventDefault()
    window.addEventListener('mousemove', resize)
    window.addEventListener('mouseup', () => {
      stopResize()
    })
  }
  const resize = (e: MouseEvent) => {
    switch (props.direction) {
      case 'left':
      default: {
        container.value!.style.width = container.value!.getBoundingClientRect().right - e.pageX + 'px'
        break
      }
      case 'right': {
        container.value!.style.width = e.pageX - container.value!.getBoundingClientRect().left + 'px'
        break
      }
      case 'top': {
        container.value!.style.height = container.value!.getBoundingClientRect().bottom - e.pageY + 'px'
      }
    }
  }

  const stopResize = () => {
    window.removeEventListener('mousemove', resize)
  }

  onMounted(() => {
    resizer.value!.addEventListener('mousedown', resizeHandler)
  })
  onBeforeUnmount(() => {
    stopResize()
    resizer.value!.removeEventListener('mousedown', resizeHandler)
  })
</script>

<template>
  <div
    ref="container"
    class="resizable-container"
  >
    <slot />
    <div
      ref="resizer"
      :class="['resizer', direction]"
    />
  </div>
</template>

<style lang="scss" scoped>
  .resizable-container {
    position: relative;
    width: v-bind('props.initialWidth');
    min-width: v-bind('props.minWidth');
    max-width: v-bind('props.maxWidth');
    height: v-bind('props.initialHeight');
    min-height: v-bind('props.minHeight');
    max-height: v-bind('props.maxHeight');
  }

  .resizer {
    content: '';
    position: absolute;
    z-index: 3;
    display: flex;

    &.left,
    &.right {
      top: 0;
      width: 4px;
      height: 100%;
      cursor: ew-resize;
    }

    &.left {
      left: 0;
      border-left: 1px solid transparent;
    }

    &.right {
      right: 0;
      border-right: 1px solid transparent;
    }

    &.top {
      top: 0;
      z-index: 3;
      width: 100%;
      height: 4px;
      border-top: 1px solid transparent;
      cursor: ns-resize;
    }

    &:hover {
      border-color: var(--border-3);
    }
  }
</style>
