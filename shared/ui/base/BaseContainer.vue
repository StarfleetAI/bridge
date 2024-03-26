<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup>
  import { ResizableContainer } from '../base'
</script>

<template>
  <div class="base-container">
    <div class="base-container__main">
      <slot name="main" />
    </div>
    <ResizableContainer
      v-if="$slots.additional"
      min-width="480px"
      max-width="590px"
      direction="left"
      initial-width="590px"
    >
      <div class="base-container__additional">
        <slot name="additional" />
      </div>
    </ResizableContainer>
  </div>
</template>
<style lang="scss" scoped>
  .base-container {
    width: 100%;

    @include flex(row, flex-start, stretch);
  }

  .base-container__main {
    display: flex;
    flex: 1;
    width: 60%;
    height: 100%;
  }

  .base-container__additional {
    position: relative;
    z-index: 3;
    width: 100%;
    height: 100%;
  }
</style>
