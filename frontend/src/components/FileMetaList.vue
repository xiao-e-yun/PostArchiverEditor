<script setup lang="ts">
import {Plus, Upload, X} from 'lucide-vue-next';
import type {FileMeta, PostId} from 'post-archiver';
import {useFileDialog} from '@vueuse/core';
import Button from './ui/button/Button.vue';
import {Separator} from './ui/separator';
import {getFileMetaPath} from '@/utils';

export interface FileMetaListProps {
  fileMetas: FileMeta[],
  post: PostId;
}

const props = defineProps<FileMetaListProps>()

const {open, onChange} = useFileDialog({multiple: true});
onChange(async (files) => {
  if (!files?.length) return;

  const formData = new FormData();
  for (const file of files) {
    formData.append('files', file);
  }
  formData.append('post', props.post.toString());
  await fetch('/api/file-meta', {method: 'POST', body: formData}).then(res => {
    if (!res.ok) throw new Error('Failed to upload files');
    return res.json();
  }).then((newFileMetas: FileMeta[]) => {
    props.fileMetas.push(...newFileMetas);
  });
});

async function deleteFiles(id: number) {
  if (!confirm('Are you sure you want to delete this file?')) return;
  await fetch(`/api/file-meta/${id}`, {method: 'DELETE'}).then(res => {
    if (!res.ok) throw new Error('Failed to delete file meta');
  });
  props.fileMetas.splice(props.fileMetas.findIndex(fm => fm.id === id), 1);
}
</script>

<template>
  <div class="flex items-center gap-2 justify-between w-full">
    <p>
      Files
      <span class="text-sm text-muted-foreground">({{ fileMetas.length }})</span>
    </p>
    <Separator class="flex-1" />
    <Button variant="outline" size="sm" @click="open">
      <Upload />
    </Button>
  </div>
  <ul class="flex flex-col gap-2">
    <li v-for="fileMeta in fileMetas" :key="fileMeta.id" class="flex items-center gap-2 h-12
      rounded-md border pr-2">
      <a v-if="fileMeta.mime.startsWith('image/')" :href="getFileMetaPath(fileMeta)" target="_blank"
        class="h-full aspect-square overflow-hidden rounded-md border-r">
        <img v-if="fileMeta.mime.startsWith('image/')" :src="getFileMetaPath(fileMeta)"
          class="h-full aspect-square object-cover" />
      </a>
      <span class="flex-1 truncate">{{ fileMeta.filename }}</span>
      <Button variant="outline" size="icon" @click="deleteFiles(fileMeta.id)">
        <X />
      </Button>
    </li>
  </ul>
</template>
