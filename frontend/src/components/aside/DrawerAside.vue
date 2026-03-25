<script setup lang="ts">
import {breakpointsTailwind, useBreakpoints} from '@vueuse/core';
import {Drawer, DrawerContent, DrawerTrigger} from '../ui/drawer';
import {PanelRightOpen} from 'lucide-vue-next';
import {Button} from '../ui/button';

const isDesktop = useBreakpoints(breakpointsTailwind).greater('lg')
</script>

<template>
  <div v-if="isDesktop" class="flex-2 h-fit sticky top-4" v-bind="$attrs">
    <slot />
  </div>
  <Drawer v-else direction="right">
    <DrawerTrigger as-child>
      <Button variant="secondary" class="fixed bottom-4 right-4 z-20 p-0">
        <PanelRightOpen />
      </Button>
    </DrawerTrigger>
    <DrawerContent v-bind="$attrs">
      <div class="p-4 h-full overflow-x-hidden overflow-y-auto flex flex-col gap-4">
        <slot />
      </div>
    </DrawerContent>
  </Drawer>
</template>
