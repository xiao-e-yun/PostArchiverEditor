<script setup lang="ts">
import ModifyButtons from '@/components/input/ModifyButtons.vue';
import PlatformSelect from '@/components/platform/PlatformSelect.vue';
import PostList from '@/components/post/list/PostList.vue';
import {Input} from '@/components/ui/input';
import {useChangeable, useCommonSaveAPI, useDifference, usePlatforms, useRemoveAPI} from '@/lib/utils';
import type {WithRelations} from '@/types';
import {useFetch} from '@vueuse/core';
import type {Tag} from 'post-archiver';
import {computed} from 'vue';
import {useRoute} from 'vue-router';

const route = useRoute();
const id = computed(() => route.params.id)
const url = computed(() => `/api/tags/${id.value}`);

const {data: raw} = useFetch(url, {refetch: true}).json<WithRelations<Tag>>();

const changed = useChangeable(raw)
const difference = useDifference(raw, changed)

const platforms = usePlatforms(changed)

const save = useCommonSaveAPI('tag', url, raw, changed, difference)
const remove = useRemoveAPI('tag', url)
</script>

<template>
  <div v-if="changed" class="flex gap-4 p-4 max-md:flex-col">
    <div class="flex-4 w-full flex flex-col gap-4">
      <Input v-model="changed.name" class="text-2xl"/>
      <PlatformSelect nullable v-model="changed.platform" :platforms="platforms" />
      <ModifyButtons :disabled="!difference" @delete="remove" @save="save" />
    </div>
    <div class="flex-2 overflow-auto flex flex-col gap-4">
      <PostList :tag="changed.id" />
    </div>
  </div>
</template>
