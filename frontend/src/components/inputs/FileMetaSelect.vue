<script setup lang="ts">
import {getFileMetaPath, isImage} from '@/utils';
import type {FileMeta} from '@api/FileMeta';
import type {FileMetaId} from '@api/FileMetaId';
import type {PostId} from '@api/PostId';
import {computed} from 'vue';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '../ui/select';

export interface FileMetaInputProps {
  fileMetas: Map<FileMetaId, FileMeta>;
  accepts?: "image" | "all";
  post?: PostId;
}

const props = defineProps<FileMetaInputProps>();

const model = defineModel<FileMetaId | null>();
const filteredFileMetas = computed(() => {
  return Array.from(props.fileMetas.values()).filter(fm => {
    const matchesPost = props.post ? fm.post === props.post : true;
    const matchesType = props.accepts === 'image' ? isImage(fm.mime) : true;
    return matchesPost && matchesType;
  });
});
</script>

<template>
  <Select v-model="model">
    <SelectTrigger class="mt-2" v-bind="$attrs">
      <SelectValue />
    </SelectTrigger>
    <SelectContent>
      <SelectItem v-for="fileMeta in filteredFileMetas" :key="fileMeta.id" :value="fileMeta.id">
        <img v-if="isImage(fileMeta.mime)" :src="getFileMetaPath(fileMeta) + '&w=128&h=128'" class="inline-block h-6
          aspect-square rounded-md border object-cover" />
        {{ fileMeta.filename }}
      </SelectItem>
    </SelectContent>
  </Select>
</template>
