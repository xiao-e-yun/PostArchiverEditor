<script setup lang="ts">
import FileMetaSelect, {type FileMetaInputProps} from './FileMetaSelect.vue';
import {getFileMetaPath} from '@/utils';

defineProps<FileMetaInputProps>();
const model = defineModel<number | null>();
</script>

<template>
  <div class="w-full relative" >
    <img v-if="model" :src="getFileMetaPath(fileMetas.get(model)!)"
      class="rounded-md border w-full h-full max-h-[32em] max-w-fit mx-auto object-cover" />
    <FileMetaSelect v-model="model" :post="post" :accepts="accepts" :file-metas="fileMetas"
    class="absolute bottom-2 left-2 bg-muted!" />
  </div>
</template>
