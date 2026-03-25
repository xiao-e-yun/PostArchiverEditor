<template>
  <Button variant="outline" @click="trigger" :class="{'is-active': isActive()}"
    class="[&.is-active]:bg-secondary">
    <slot />
  </Button>
</template>

<script setup lang="ts">
import {inject} from 'vue';
import {key} from '.';
import type {ChainedCommands} from '@tiptap/core';
import {Button} from '@/components/ui/button';

const props = defineProps<{
  name: string,
  attribute?: Record<string, any>,
  trigger: (chainedCommands: ChainedCommands) => ChainedCommands,
}>()

const editor = inject(key)!;

function trigger() {
  if (!editor.value) return;
  const commands = editor.value.chain().focus()
  props.trigger(commands).run();
}

function isActive() {
  if (!editor.value) return false;
  return editor.value.isActive(props.name, props.attribute);
}
</script>
