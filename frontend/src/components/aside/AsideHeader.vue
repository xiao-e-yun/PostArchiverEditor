<script setup lang="ts">
const value = defineModel<string>()
</script>

<template>
  <div class="border-b font-bold text-center text-sm relative h-12 lg:h-8 flex-shrink-0">
    <input v-model="value" class="w-full h-full py-1 px-2 max-md:px-4" placeholder="Search..." />
  </div>
</template>
