<script setup lang="ts">
import LayoutHeader from './components/layout/Header.vue';
import LayoutAside from './components/layout/Aside.vue';
import LayoutMain from './components/layout/Main.vue';
import Settings from './components/Settings.vue';
import {useActiveTab} from './utils';

const activeTab = useActiveTab();
</script>

<template>
  <LayoutHeader />
  <Settings v-if="activeTab === 'settings'" />
  <div v-else class="flex flex-1 overflow-hidden">
    <LayoutAside class="flex-shrink-0" />
    <LayoutMain class="overflow-auto" />
  </div>
</template>

<style>
#app {
  height: 100vh;
  display: flex;
  flex-direction: column;
}
</style>
