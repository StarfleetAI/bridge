<!-- Copyright 2024 StarfleetAI -->
<!-- SPDX-License-Identifier: Apache-2.0 -->
<script setup lang="ts">
  import { ChevronLeftIcon as ChevronLeftIconAsync, ChevronRightIcon as ChevronRightIconAsync } from '../../icons'

  const ChevronLeftIcon = defineAsyncComponent(ChevronLeftIconAsync)
  const ChevronRightIcon = defineAsyncComponent(ChevronRightIconAsync)
  const props = defineProps<{
    totalPages: number
    maxPageVisible?: number
  }>()

  const currentPage = defineModel<number>({ default: 1 })

  const pages = computed(() => {
    const pagesArr: (number | '...')[] = []
    let startPage = Math.max(currentPage.value - Math.floor((props.maxPageVisible || 5) / 2), 1)
    let endPage = startPage + (props.maxPageVisible || 5) - 1

    if (endPage > props.totalPages) {
      endPage = props.totalPages
      startPage = Math.max(1, endPage - (props.maxPageVisible || 5) + 1)
    }

    for (let i = startPage; i <= endPage; i++) {
      pagesArr.push(i)
    }

    if (startPage > 1) {
      pagesArr.splice(1, 0, '...')
      pagesArr.splice(0, 1, 1)
    }

    if (endPage < props.totalPages) {
      pagesArr.splice(pagesArr.length - 1, 0, '...')
    }

    return pagesArr
  })

  const selectPage = (page: number | '...') => {
    if (page !== '...' && page !== currentPage.value) {
      currentPage.value = page
    }
  }
</script>
<template>
  <nav aria-label="Page navigation">
    <ul class="pagination">
      <li
        class="page-item"
        :class="{ disabled: currentPage <= 1 }"
        @click="selectPage(currentPage - 1)"
      >
        <ChevronLeftIcon />
      </li>
      <template v-for="page in pages">
        <li
          v-if="page !== '...'"
          class="page-item"
          :class="{ active: page === currentPage }"
          @click="selectPage(page as number)"
        >
          {{ page }}
        </li>
        <li
          v-else
          class="page-item disabled"
        >
          ...
        </li>
      </template>
      <li
        class="page-item"
        :class="{ disabled: currentPage >= totalPages }"
        @click="selectPage(currentPage + 1)"
      >
        <ChevronRightIcon />
      </li>
    </ul>
  </nav>
</template>

<style lang="scss" scoped>
  .pagination {
    padding-left: 0;
    list-style: none;

    @include flex(row, center, center);
  }

  .page-item {
    width: 40px;
    height: 40px;
    margin-bottom: 0;
    border-radius: 50%;
    text-decoration: none;

    @include flex(row, center, center);
    @include font-inter-500(12px, 17px, var(--text-tertiary));
  }

  .page-item.active,
  .page-item:hover {
    background-color: var(--surface-2);
    color: var(--text-primary);
  }

  .page-item.disabled {
    opacity: 0.5;
    pointer-events: none;
  }
</style>
