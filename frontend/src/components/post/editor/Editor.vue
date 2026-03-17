<template>
  <div class="border m-4 rounded-lg p-2 flex flex-col gap-8">
    <EditorHeader />
    <EditorMenu />
    <EditorDragHandle />
    <EditorContent :editor="editor" class="mx-12 prose prose-stone dark:prose-invert [&>.tiptap>:first-child]:mt-0
           [&>.tiptap>:last-child]:mb-0 [&_*]:outline-none" />
  </div>
  <code class="whitespace-pre-line" v-html="editor?.getMarkdown()" />
</template>

<script setup lang="ts">
import {onUnmounted, provide} from 'vue';

import {useEditor, EditorContent} from '@tiptap/vue-3'

import Typography from '@tiptap/extension-typography'
import Highlight from '@tiptap/extension-highlight'
import StarterKit from '@tiptap/starter-kit'

import {Markdown} from '@tiptap/markdown'

import EditorMenu from './EditorMenu.vue';

import {key} from '.';
import EditorDragHandle from './EditorDragHandle.vue';
import EditorHeader from './EditorHeader.vue';

const editor = useEditor({
  content: "<p>I'm running Tiptap with Vue.js. 🎉</p>",
  contentType: 'markdown',
  extensions: [StarterKit, Highlight, Typography, Markdown],
});

provide(key, editor);
onUnmounted(() => editor.value?.destroy());
</script>
