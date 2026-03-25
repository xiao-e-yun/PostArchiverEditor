<template>
  <div class="lg:border lg:rounded-lg lg:p-2">
    <EditorHeader />
    <EditorMenu />
    <EditorDragHandle />
    <EditorContent :editor="editor" :class="$style.tiptap" class="max-w-none my-4 ml-10 mr-4 prose prose-stone dark:prose-invert [&>.tiptap>:first-child]:mt-0
           [&>.tiptap>:last-child]:mb-0 [&_*]:outline-none" />
  </div>
</template>

<script setup lang="ts">
import PlaceholderImage from '@/assets/placeholder.svg';

import {onMounted, onUnmounted, provide, watch} from 'vue';

import {useEditor, EditorContent} from '@tiptap/vue-3'

import Typography from '@tiptap/extension-typography'
import Highlight from '@tiptap/extension-highlight'
import Image from '@tiptap/extension-image'
import StarterKit from '@tiptap/starter-kit'

import {Markdown} from '@tiptap/markdown'

import EditorMenu from './EditorMenu.vue';

import {insertFileKey, key} from '.';
import EditorDragHandle from './EditorDragHandle.vue';
import EditorHeader from './EditorHeader.vue';
import {isNumber, isString} from 'lodash-es';
import type {Content, FileMeta} from 'post-archiver';
import {parseFileMetaToUrl} from '@/lib/utils';
import {useEventBus} from '@vueuse/core';

const props = defineProps<{
  fileMetas: Record<number, FileMeta>
}>()

const editor = useEditor({
  content: "",
  contentType: 'markdown',
  extensions: [
    Markdown,
    StarterKit.configure({
      link: {
        openOnClick: false,
      }
    }),
    Highlight,
    Typography,
    Image.extend({
      renderHTML({HTMLAttributes}) {
        const filename = (HTMLAttributes.alt as string).split(";")[1];
        HTMLAttributes.title = filename;

        if (HTMLAttributes.src !== PlaceholderImage) return ["img", HTMLAttributes]

        return [
          "div", {class: "image-wrapper"},
          ["div", {class: "file-preview"},
            ["img", HTMLAttributes],
            ["span", filename]
          ]
        ]
      },
    })
  ],
  onUpdate({editor}) {
    const md = editor.getMarkdown();
    rawContent.value = md.split("\n\n").map((line) => {
      // ![ID:FILE_META_ID;FILENAME](URL)
      const match = line.match(/!\[ID:(\d+);(.+)\]\((.+)\)/);
      if (match) {
        const fileMetaId = match[1];
        if (isNumber(Number(fileMetaId))) return Number(fileMetaId);
      }
      return line;
    });
  }
});

const rawContent = defineModel<Content[]>()
watch(rawContent, updateContents)

function updateContents(content?: Content[]) {
  const $editor = editor.value;
  if (!$editor) return;

  const $content = content?.map((block) => {
    if (isString(block)) return block;

    const fileMeta = props.fileMetas[block];
    if (!fileMeta) return "[Missing file]";

    const isImage = fileMeta.mime.startsWith("image/");
    const alt = `ID:${block};${fileMeta.filename}`;

    const url = isImage ? parseFileMetaToUrl(fileMeta) : PlaceholderImage;
    return `![${alt}](${url})`;
  }).join("\n\n") ?? ""

  $editor
    .chain()
    .setMeta('addToHistory', false)
    .setContent($content, {contentType: "markdown"})
    .run();
}

provide(key, editor);
onMounted(() => updateContents(rawContent.value));
onUnmounted(() => editor.value?.destroy());

useEventBus(insertFileKey).on(fileMeta => {
  console.log("Inserting file: ", fileMeta)

  const isImage = fileMeta.mime.startsWith("image/");
  const src = isImage ? parseFileMetaToUrl(fileMeta) : PlaceholderImage;
  const alt = `ID:${fileMeta.id}`;

  editor
    .value
    ?.chain()
    .focus()
    .insertContent({
      type: "image",
      attrs: {src, alt}
    }
    ).run()
})
</script>

<style module>
.tiptap :global(.ProseMirror-selectednode) {
  outline: 2px solid var(--secondary);
}

.tiptap img {
  width: 100%;
  max-height: 30rem;
  object-fit: contain;
}


.tiptap :global(.image-wrapper) {
  width: 100%;
  display: flex;
  justify-content: center;
}

.tiptap :global(.file-preview) {
  border-radius: 0.5rem;
  background: var(--secondary);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem 1rem;
}

.tiptap :global(.file-preview)>img {
  object-fit: contain;
  max-width: 100%;
  height: 10rem;
  margin: 0;
}

:global(.dark) .tiptap :global(.file-preview)>img {
  filter: invert(1);
}
</style>
