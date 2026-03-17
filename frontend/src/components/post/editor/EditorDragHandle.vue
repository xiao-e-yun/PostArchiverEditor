<template>
  <DragHandle v-if="editor" :editor="editor" :nested="{edgeDetection: {threshold: -16}}">
    <GripVertical class="cursor-grab hover:bg-secondary rounded my-0.5 mr-2" />
  </DragHandle>
</template>

<script setup lang="ts">
import {inject} from 'vue';
import {key} from '.';

import {DragHandle} from '@tiptap/extension-drag-handle-vue-3'

import {GripVertical} from "lucide-vue-next";

const editor = inject(key)!;
</script>
