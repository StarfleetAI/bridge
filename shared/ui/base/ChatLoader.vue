<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

<script lang="ts" setup></script>

<template>
  <div class="lds-ring">
    <div />
    <div />
    <div />
    <div />
  </div>
</template>

<style lang="scss" scoped>
  .lds-ring {
    position: relative;
    display: inline-block;
    width: 16px;
    height: 16px;
  }

  .lds-ring div {
    position: absolute;
    display: block;
    box-sizing: border-box;
    width: 16px;
    height: 16px;
    margin: 0;
    border: 2px solid var(--button-primary);
    border-color: var(--button-primary) transparent transparent;
    border-radius: 50%;
    animation: lds-ring 1.2s cubic-bezier(0.5, 0, 0.5, 1) infinite;
  }

  .lds-ring div:nth-child(1) {
    animation-delay: -0.45s;
  }

  .lds-ring div:nth-child(2) {
    animation-delay: -0.3s;
  }

  .lds-ring div:nth-child(3) {
    animation-delay: -0.15s;
  }

  @keyframes lds-ring {
    0% {
      transform: rotate(0deg);
    }

    100% {
      transform: rotate(360deg);
    }
  }
</style>
