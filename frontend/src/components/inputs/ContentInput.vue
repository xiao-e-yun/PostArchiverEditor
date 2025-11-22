<script setup lang="ts">
import {commitRef} from '@/utils';
import type {Content} from '@api/Content';
import FileMetaInput from './FileMetaInput.vue';
import type {FileMeta} from '@api/FileMeta';
import {isString, last} from 'lodash-es';
import {Textarea} from '../ui/textarea';
import {Button} from '../ui/button';
import {computed, ref} from 'vue';

const rawContents = defineModel<Content[]>({ default: () => [] });
const contents = commitRef(computed({
  get: () => cleanupContent(rawContents.value),
  set: (val) => rawContents.value = cleanupContent(val)
}));

defineProps<{
  fileMetas: Map<number, FileMeta>;
  post: number;
}>();

function cleanupContent(contents: Content[]) {
  return contents.reduce((acc, curr) => {
    if (isString(last(acc)) && isString(curr)) acc.push(acc.pop() + curr);
    else acc.push(curr);
    return acc;
  }, [] as (string | number)[]).filter(c => !isString(c) || c.trim() !== '')
}

const changed = ref(false);
const commitContent = () => {
  contents.commit()
  changed.value = false;
}
</script>

<template>
  <div class="border rounded-md relative flex flex-col gap-2 p-2">
    <template v-for="content, i in contents" :key="i">
      <Textarea v-if="isString(content)" v-model="contents[i]" class="w-full resize-none min-h-auto" @update:model-value="changed = true" />
      <FileMetaInput v-else v-model="contents[i] as number" :post="post" :file-metas="fileMetas" @update:model-value="changed = true" />
    </template>
    <div  class="sticky bottom-0 ml-auto h-0">
      <Button v-if="changed" @click="commitContent" variant="secondary" class="block absolute bottom-0 right-0">Save</Button>
    </div>
  </div>
</template>
