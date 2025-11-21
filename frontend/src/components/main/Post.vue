<script setup lang="ts">
import {commitRef, getFileMetaPath, reactiveChanges, setActiveItem, useRelations} from '@/utils';
import type {PostResponse} from '@api/PostResponse';
import type {WithRelations} from '@api/WithRelations';
import {computed, ref, toRef} from 'vue';
import ImageInput from '../inputs/ImageInput.vue';
import {Input} from '../ui/input';
import {Badge} from '../ui/badge';
import DateTimeInput from '../inputs/DateTimeInput.vue';
import {isEmpty, isString, last} from 'lodash-es';
import {Button} from '../ui/button';
import type {Content} from '@api/Content';
import {FilesIcon} from 'lucide-vue-next';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '../ui/select';
import {Textarea} from '../ui/textarea';

const props = defineProps<{
  data: WithRelations<PostResponse>
}>();

const data = toRef(props, 'data');
const relations = computed(() => useRelations(props.data));

const proxyed = ref(reactiveChanges(data.value));

const proxyedSource = computed({
  get: () => proxyed.value.source || '',
  set: (val: string) => proxyed.value.source = isEmpty(val) ? null : val
})

const cleanupContent = (contents: Content[]) => {
  return contents.reduce((acc, curr) => {
    if (isString(last(acc)) && isString(curr)) acc.push(acc.pop() + curr);
    else acc.push(curr);
    return acc;
  }, [] as (string | number)[]).filter(c => !isString(c) || c.trim() !== '')
}
const proxyedContents = commitRef(computed({
  get: () => cleanupContent(proxyed.value.content),
  set: (val) => proxyed.value.content = cleanupContent(val)
}))
const commitContents = () => proxyedContents.commit();
</script>

<template>
  <div class="flex gap-4 max-lg:flex-col-reverse relative">
    <div class="flex flex-col gap-4 w-full">
      <Input id="title" v-model="proxyed.title" class="w-full h-max p-2 border text-2xl!" placeholder="Title" />
      <div class="border rounded-md relative flex flex-col gap-2 p-2">
        <template v-for="content, i in proxyedContents" :key="i">
          <Textarea v-if="isString(content)" v-model="proxyedContents[i]" class="w-full resize-none
            min-h-auto"/>
          <template v-else v-for="fileMeta in [relations.fileMetas.get(content)!]">
            <ImageInput v-if="fileMeta.mime.startsWith('image/')" v-model="proxyedContents[i] as number"
              :relations="relations" :post="data.id" />
            <Select v-else v-model="proxyedContents[i] as number">
              <SelectTrigger class="bg-input/30 border p-4 rounded-md aspect-square flex flex-col
              items-center justify-center hover:bg-input/50 mx-auto h-auto!">
                <FilesIcon class="w-full! h-full! flex-auto!" />
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem v-for="fileMeta in
                  Array.from(relations.fileMetas.values()).filter(f => f.post === data.id)" :key="fileMeta.id"
                  :value="fileMeta.id">
                  <img v-if="fileMeta.mime.startsWith('image/')" :src="getFileMetaPath(fileMeta) + '&w=128&h=128'"
                    class="inline-block h-6 aspect-square rounded-md border object-cover" />
                  {{ fileMeta.filename }}
                </SelectItem>
              </SelectContent>
            </Select>
          </template>
        </template>
        <Button @click="commitContents" variant="secondary" class="sticky bottom-0 m-2 ml-auto block">Save</Button>
      </div>
      {{ proxyed.changes }}
    </div>
    <div class="flex flex-col gap-4 lg:w-lg lg:sticky lg:top-0 lg:self-start">
      <ImageInput v-model="proxyed.thumb" :relations="relations" :post="data.id" />
      <Input v-model="proxyedSource" class="w-full" placeholder="Source URL" />
      <div v-if="proxyed.authors.length">
        <span class="text-sm ml-2">Authors:</span>
        <div class="border rounded-md p-2 flex flex-wrap gap-1">
          <Badge v-for="author in proxyed.authors" :key="author.id" @click="setActiveItem('authors',
            author.id)" class="cursor-pointer select-none">
            @{{ author.name }}
          </Badge>
        </div>
      </div>
      <div v-if="proxyed.tags.length">
        <span class="text-sm ml-2">Tags:</span>
        <div class="border rounded-md p-2 flex flex-wrap gap-1">
          <Badge v-for="tag in proxyed.tags" :key="tag.id" @click="setActiveItem('tags',
            tag.id)" class="cursor-pointer select-none">
            #{{ tag.name }}
            <span v-if="tag.platform" class="opacity-50">
              ({{ relations.platforms.get(tag.platform)?.name }})
            </span>
          </Badge>
        </div>
      </div>
      <div v-if="proxyed.collections.length">
        <span class="text-sm ml-2">Collections:</span>
        <div class="border rounded-md p-2 flex flex-wrap gap-1">
          <Badge v-for="collection in proxyed.collections" :key="collection.id" @click="setActiveItem('collections',
            collection.id)" class="cursor-pointer select-none" :title="collection.source">
            .{{ collection.name }}
          </Badge>
        </div>
      </div>
      <div>
        Published:
        <DateTimeInput v-model="proxyed.published" />
      </div>
      <div>
        Updated:
        <DateTimeInput v-model="proxyed.updated" />
      </div>
    </div>
  </div>
</template>
