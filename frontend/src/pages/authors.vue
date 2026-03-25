<script setup lang="ts">
import Aliases from '@/components/author/aliases/Aliases.vue';
import DateTimeInput from '@/components/input/DateTimeInput.vue';
import ModifyButtons from '@/components/input/ModifyButtons.vue';
import PostList from '@/components/post/list/PostList.vue';
import {Input} from '@/components/ui/input';
import {parseFileMetaToUrl, useChangeable, useCommonSaveAPI, useDifference, useFileMetas, useRemoveAPI} from '@/lib/utils';
import type {WithRelations} from '@/types';
import {useFetch} from '@vueuse/core';
import type {Author} from 'post-archiver';
import {computed} from 'vue';
import {useRoute} from 'vue-router';

const route = useRoute();
const id = computed(() => route.params.id)
const url = computed(() => `/api/authors/${id.value}`);

const {data: raw} = useFetch(url, {refetch: true}).json<WithRelations<Author>>();

const changed = useChangeable(raw)
const difference = useDifference(raw, changed)

const fileMetas = useFileMetas(changed)

const save = useCommonSaveAPI('author', url, raw, changed, difference)
const remove = useRemoveAPI('author', url)
</script>

<template>
  <div v-if="changed" class="flex gap-4 p-4 max-md:flex-col">
    <div class="flex-4 w-full flex flex-col gap-4">
      <Input v-model="changed.name" class="text-2xl"/>
      <DateTimeInput v-model="changed.updated" label="Updated" />
      <Aliases :author="changed.id" />
      <ModifyButtons :disabled="!difference" @delete="remove" @save="save" />
    </div>
    <div class="flex-2 overflow-auto flex flex-col gap-4">
      <img v-if="changed.thumb" :src="parseFileMetaToUrl(fileMetas[changed.thumb]!)" alt="cover" class="aspect-video object-cover rounded-md">
      <PostList :author="changed.id" />
    </div>
  </div>
</template>
