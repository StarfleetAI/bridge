<template>
  <div class="flex h-screen p-3 space-x-3">
    <!-- Sidebar -->
    <div class="bg-gray-800 rounded-2xl py-2 px-2 flex flex-col items-center justify-start space-y-2">
      <NuxtLink to="/" class="w-11 h-11 bg-gray-500 flex items-center justify-center rounded-xl font-bold">
        A
      </NuxtLink>
      <NuxtLink to="/docs" class="w-11 h-11 bg-gray-500 flex items-center justify-center rounded-xl font-bold">
        D
      </NuxtLink>
      <NuxtLink to="/settings" class="w-11 h-11 bg-gray-500 flex items-center justify-center rounded-xl font-bold">
        S
      </NuxtLink>
    </div>

    <!-- Content Area -->
    <div class="flex flex-col flex-grow">
      <h1 class="text-2xl font-bold mb-3 text-slate-400">
        {{ $route.meta.title }}
      </h1>

      <div class="bg-gray-700 rounded-2xl p-3 flex-grow">
        <NuxtPage />
      </div>
    </div>
  </div>
</template>
