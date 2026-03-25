<script setup lang="ts">
import AsideTabs from "./AsideTabs.vue";
import {RouterView} from "vue-router";
</script>

<template>
  <aside class="flex-shrink-0 border-r flex flex-col justify-between lg:w-64 ">
    <div class="flex-1 flex flex-col overflow-hidden">
      <RouterView name="aside" />
    </div>
    <AsideTabs />
  </aside>
</template>
