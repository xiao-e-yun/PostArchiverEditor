<script setup lang="ts">
import {breakpointsTailwind, useBreakpoints, useDark} from "@vueuse/core";
import Aside from "./components/aside/Aside.vue";
import {Drawer, DrawerContent, DrawerTrigger} from "./components/ui/drawer";
import {Button} from "./components/ui/button";
import {Menu, Moon, Sun} from "lucide-vue-next";
import {useRoute} from "vue-router";
import {computed} from "vue";
import {isInteger} from "lodash-es";
import {Toaster} from "./components/ui/sonner";

const dark = useDark();

const isDesktop = useBreakpoints(breakpointsTailwind).greater('lg')

const route = useRoute();
const isFallbackPage = computed(() => route.meta.requireId && !isInteger(parseInt(route.params.id as string)))
</script>

<template>
  <Toaster position="top-center" />

  <div class="h-dvh w-dvw flex overflow-x-hidden max-lg:flex-col">

    <Aside v-if="isDesktop" />
    <div class="flex-1 flex flex-col h-full overflow-hidden">

      <header class="w-full h-12 lg:h-8 border-b flex items-center px-4 lg:pr-1 gap-2">
        <span class="font-bold text-sm flex-1">Post Archiver Editor</span>
        <Button variant="ghost" @click="dark = !dark" class="lg:w-8 lg:h-7">
          <Sun class="dark:hidden" />
          <Moon class="hidden dark:inline" />
        </Button>
        <Drawer v-if="!isDesktop" direction="left">
          <DrawerTrigger as-child>
            <Button variant="ghost">
              <Menu />
            </Button>
          </DrawerTrigger>
          <DrawerContent class="w-full overflow-x-hidden overflow-y-auto">
            <Aside class="h-full" />
          </DrawerContent>
        </Drawer>
      </header>


      <main class="flex-1 overflow-auto">
        <RouterView v-if="!isFallbackPage" />
        <div v-else class="h-full flex items-center justify-center">
          <span class="text-muted-foreground text-sm">Select an item from the sidebar to get started</span>
        </div>
      </main>
    </div>
  </div>
</template>
