<script setup lang="ts">
import type {FileMetaInputProps} from './FileMetaSelect.vue';
import ImageInput from './ImageInput.vue';
import FileInput from './FileInput.vue';
import {isImage} from '@/utils';

const model = defineModel<number | null>();
defineProps<FileMetaInputProps>();
</script>

<template>
  <ImageInput v-if="isImage(fileMetas.get(model as number)?.mime)" v-model="model" :post="post" :accepts="accepts" :file-metas="fileMetas" />
  <FileInput v-else v-model="model" :post="post" :accepts="accepts" :file-metas="fileMetas" />
</template>
