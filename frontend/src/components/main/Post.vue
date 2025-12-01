<script setup lang="ts">
import {commitRef, getFileMetaPath, reactiveChanges, setActiveItem, useRelations} from '@/utils';
import type {PostResponse} from '@api/PostResponse';
import type {WithRelations} from '@api/WithRelations';
import {computed, ref, toRef} from 'vue';
import ImageInput from '../inputs/ImageInput.vue';
import {Input} from '../ui/input';
import {Badge} from '../ui/badge';
import DateTimeInput from '../inputs/DateTimeInput.vue';
import {isEmpty, remove} from 'lodash-es';
import ContentInput from '../inputs/ContentInput.vue';
import CategoryGroupInput from '../inputs/CategoryGroupInput.vue';
import { CategoryType } from '@/category';

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
</script>

<template>
  <div class="flex gap-4 max-lg:flex-col-reverse relative">
    <div class="flex flex-col gap-4 w-full">
      <Input id="title" v-model="proxyed.title" class="w-full h-max p-2 border text-2xl!" placeholder="Title" />
      <ContentInput v-model="proxyed.content" :post="data.id" :file-metas="relations.file_metas" class="w-full" />
      {{ proxyed.changes }}
    </div>
    <div class="flex flex-col gap-4 lg:w-lg lg:sticky lg:top-0 lg:self-start">
      <ImageInput v-model="proxyed.thumb" :post="data.id" :file-metas="relations.file_metas" accepts="image" />
      <Input v-model="proxyedSource" class="w-full" placeholder="Source URL" />
      <div>
        Published:
        <DateTimeInput v-model="proxyed.published" />
      </div>
      <div>
        Updated:
        <DateTimeInput v-model="proxyed.updated" />
      </div>
      <CategoryGroupInput 
        v-model="proxyed.authors"
        :type="CategoryType.Author"
        :relations="relations"
      />
      <div>
        <span class="text-sm ml-2">
          Authors:
        </span>
        <div class="border rounded-md p-2 flex flex-wrap gap-1">
          <Badge v-for="author in proxyed.authors" :key="author.id" @click="remove(proxyed.authors,
                        author)
          " class="cursor-pointer select-none">
            @{{ author.name }}
          </Badge>
          <div v-if="!proxyed.authors.length" class="opacity-50">
            No authors assigned.
          </div>
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
    </div>
  </div>
</template>
