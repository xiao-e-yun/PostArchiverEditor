<script setup lang="ts">
import {FilesIcon} from 'lucide-vue-next';
import type {FileMetaInputProps} from './FileMetaSelect.vue';
import FileMetaSelect from './FileMetaSelect.vue';

defineProps<FileMetaInputProps>()
const model = defineModel<number | null>()

</script>

<template>
  <div class="bg-input/30 border p-4 rounded-md aspect-square flex flex-col
    items-center justify-center hover:bg-input/50 mx-auto relative">
    <FilesIcon class="w-full! h-full! flex-auto!" />
    <FileMetaSelect v-model="model" :post="post" :accepts="accepts" :file-metas="fileMetas" />
  </div>
</template>
