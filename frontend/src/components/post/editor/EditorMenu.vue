<template>
  <BubbleMenu v-if="editor" :editor="editor">
    <ButtonGroup>
      <EditorButton name="bold" :trigger="e => e.toggleBold()">
        <Bold />
      </EditorButton>
      <EditorButton name="italic" :trigger="e => e.toggleItalic()">
        <Italic />
      </EditorButton>
      <EditorButton name="strikethrough" :trigger="e => e.toggleStrike()" variant="outline">
        <Strikethrough />
      </EditorButton>
      <EditorButton name="code" :trigger="e => e.toggleCode()" variant="outline">
        <Code />
      </EditorButton>
      <EditorButton name="highlight" :trigger="e => e.toggleHighlight()">
        <Highlighter />
      </EditorButton>
    </ButtonGroup>
  </BubbleMenu>
</template>

<script setup lang="ts">
import {ButtonGroup} from '@/components/ui/button-group'
import {BubbleMenu} from '@tiptap/vue-3/menus'

import EditorButton from './EditorButton.vue';

import {inject} from 'vue';
import {key} from '.';
import {Bold, Highlighter, Italic, Strikethrough, Code} from 'lucide-vue-next';

const editor = inject(key)!;
</script>
