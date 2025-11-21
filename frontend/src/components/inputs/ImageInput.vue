<script setup lang="ts">
import {getFileMetaPath, useRelations} from '@/utils';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '../ui/select';
import {computed} from 'vue';
import type {PostId} from '@api/PostId';

const model = defineModel<number | null>();
const props = defineProps<{
  relations: ReturnType<typeof useRelations>;
  onlyImages?: boolean;
  post?: PostId
}>();

const isSamePost = (post: PostId) => props.post && post === props.post;
const isImage = (mime: string) => mime.startsWith('image/');
const acceptType = (mime: string) => !props.onlyImages || isImage(mime);
const selectableFileMetas = computed(() => Array.from(props.relations.fileMetas.values()).filter(fm => isSamePost(fm.post) && acceptType(fm.mime)));
</script>

<template>
  <div class="w-full relative">
    <img v-if="model" :src="getFileMetaPath(props.relations.fileMetas.get(model)!)"
      class="rounded-md border w-full h-full max-h-[32em] max-w-fit mx-auto object-cover" />
    <Select v-model="model">
      <SelectTrigger class="absolute bottom-2 left-2 backdrop-brightness-30 backdrop-blur-sm">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem v-for="fileMeta in selectableFileMetas" :key="fileMeta.id" :value="fileMeta.id">
          <img v-if="isImage(fileMeta.mime)" :src="getFileMetaPath(fileMeta) + '&w=128&h=128'" class="inline-block h-6
          aspect-square rounded-md border object-cover" />
          {{ fileMeta.filename }}
        </SelectItem>
      </SelectContent>
    </Select>
  </div>
</template>
