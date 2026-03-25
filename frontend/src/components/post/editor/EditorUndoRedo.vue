<template>
  <ButtonGroup>
    <Button variant="outline" @click="undo" :disabled="!canUndo">
      <Undo />
    </Button>
    <Button variant="outline" @click="redo" :disabled="!canRedo">
      <Redo />
    </Button>
  </ButtonGroup>
</template>

<script setup lang="ts">
import {computed, inject} from 'vue';
import {key} from '.';
import {Button} from '@/components/ui/button';
import {ButtonGroup} from '@/components/ui/button-group';
import {Redo, Undo} from 'lucide-vue-next';

const editor = inject(key)!;

const undo = () => editor.value?.chain().focus().undo().run();
const redo = () => editor.value?.chain().focus().redo().run();
const canUndo = computed(() => editor.value?.can().undo() ?? false);
const canRedo = computed(() => editor.value?.can().redo() ?? false);
</script>
