<script setup lang="ts">
import LayoutHeader from './components/layout/Header.vue';
import LayoutAside from './components/layout/Aside.vue';
import LayoutMain from './components/layout/Main.vue';
import Settings from './components/Settings.vue';
import {useSettingsTab} from './utils';
import {TooltipProvider} from './components/ui/tooltip';
import {Toaster} from './components/ui/sonner';

import 'vue-sonner/style.css'

const settingsTab = useSettingsTab();
</script>

<template>
  <TooltipProvider>
    <Toaster />
    <LayoutHeader />
    <Settings v-if="settingsTab" />
    <div v-else class="flex flex-1 overflow-hidden">
      <LayoutAside class="shrink-0" />
      <LayoutMain class="overflow-auto" />
    </div>
  </TooltipProvider>
</template>

<style>
#app {
  height: 100vh;
  display: flex;
  flex-direction: column;
}
</style>
