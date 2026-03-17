<template>
  <div class="flex items-center gap-2">
    <ButtonGroup>
      <EditorButton name="heading" :attribute="{level: 1}" :trigger="e => e.toggleHeading({level: 1})">
        <Heading1 />
      </EditorButton>
      <EditorButton name="heading" :attribute="{level: 2}" :trigger="e => e.toggleHeading({level: 2})">
        <Heading2 />
      </EditorButton>
      <EditorButton name="heading" :attribute="{level: 3}" :trigger="e => e.toggleHeading({level: 3})">
        <Heading3 />
      </EditorButton>
      <EditorButton name="paragraph" :trigger="e => e.setParagraph()">
        <Pilcrow />
      </EditorButton>
    </ButtonGroup>
    <ButtonGroup>
      <EditorButton name="bulletList" :trigger="e => e.toggleBulletList()">
        <List />
      </EditorButton>
      <EditorButton name="orderedList" :trigger="e => e.toggleOrderedList()">
        <ListOrdered />
      </EditorButton>
      <EditorButton name="blockquote" :trigger="e => e.toggleBlockquote()">
        <Quote />
      </EditorButton>
      <EditorButton name="codeBlock" :trigger="e => e.toggleCodeBlock()">
        <SquareCode />
      </EditorButton>
    </ButtonGroup>
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
      <EditorLinkSelect />
    </ButtonGroup>
  </div>
</template>

<script setup lang="ts">
import {ButtonGroup} from '@/components/ui/button-group'

import EditorButton from './EditorButton.vue';

import type {Editor} from '@tiptap/vue-3';
import {Bold, Code, Heading1, Heading2, Heading3, Highlighter, Italic, List, ListOrdered, Pilcrow, Quote, SquareCode, Strikethrough} from 'lucide-vue-next';
import EditorLinkSelect from './EditorLinkSelect.vue';
</script>
