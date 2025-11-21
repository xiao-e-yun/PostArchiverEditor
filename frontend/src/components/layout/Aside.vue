<script setup lang="ts">
import {getFileMetaPath, getUrlWithParams, useActiveItem, useActiveTab, useRelations} from '@/utils';
import {Input} from '../ui/input';
import {refDebounced, useInfiniteScroll, useSessionStorage, useVirtualList} from '@vueuse/core';
import {ref, useTemplateRef, watch} from 'vue';
import {Alert, AlertDescription, AlertTitle} from '../ui/alert';
import type {ListResponse, WithRelations} from '@/api';
import {ScrollArea} from '../ui/scroll-area';
import {LoaderCircleIcon} from 'lucide-vue-next';

const activeTab = useActiveTab();
const activeItem = useActiveItem();

const search = useSessionStorage('editor-search', '');
const searchDebounced = refDebounced(search, 250);

let page: number | null = 0;
const error = ref<[string, string] | null>(null);
const list = ref<{
  id: number;
  name: string;
  thumb: string | null;
}[]>([]);
const listEl = useTemplateRef<HTMLDivElement>('lastEl');
const {reset, isLoading} = useInfiniteScroll(listEl, async () => {
  if (page === null) return

  error.value = null;
  let response: Response;
  try {
    response = await fetch(getUrlWithParams(`/api/${activeTab.value}`, {
      search: searchDebounced.value,
      page: page || 0
    }));
  } catch (e) {
    error.value = ['Network Error', (e as Error).message];
    page = null;
    return
  }

  let result: WithRelations<ListResponse>;
  try {
    result = await response.json();
    page = result.list.length === 20 ? (page || 0) + 1 : null;
  } catch (e) {
    error.value = ['Error parsing response', (e as Error).message];
    page = null;
    return
  }

  const relations = useRelations(result);
  list.value.push(...result.list.map(item => {

    let thumb = null;
    if (activeTab.value === 'file_metas') {
      const exts = ['webp', 'png', 'jpg', 'jpeg', 'gif', 'bmp', 'tiff', 'svg'];
      const ext = item.name.split('.').pop()!.toLowerCase();
      if (exts.includes(ext)) {
        const [post, filename] = item.name.split('/') as [string, string];
        thumb = `/images/${Math.floor(parseInt(post) / 2048)}/${parseInt(post) % 2048}/${filename}`
      }
    } else if (item.thumb) thumb = relations.fileMetaPath(item.thumb)!;


    return ({
      id: item.id,
      name: item.name,
      thumb
    })
  }))
}, {
  distance: 20,
  canLoadMore: () => page !== null,
});

watch([activeTab, searchDebounced], resetList);
function resetList() {
  list.value.length = 0;
  page = 0;
  reset();
}

const {list: vList, containerProps, wrapperProps} = useVirtualList(list, {itemHeight: 65, overscan: 5});
</script>

<template>
  <aside class="w-32 md:w-60 border-r bg-secondary h-full overflow-hidden">
    <Input class="rounded-none border-none border-b p-1" placeholder="Search..." v-model="search" />
    <ScrollArea class="h-[calc(100%-36px)]">
      <Alert v-if="error" variant="destructive" class="mx-auto my-2 w-fit">
        <AlertTitle>{{ error[0] }}</AlertTitle>
        <AlertDescription>{{ error[1] }}</AlertDescription>
      </Alert>
      <div v-else-if="list.length === 0" class="p-4">No items found.</div>
      <div v-bind="containerProps">
        <ul v-bind="wrapperProps" class="m-0 p-0 list-none">
          <li v-for="{data: item, index} in vList" :key="index" @click="activeItem = {id: item.id, type: activeTab}"
            class="flex items-center hover:bg-input/30
            cursor-pointer border-b relative p-2 flex flex-col items-start" :class="{
              'bg-input/30':
                activeItem?.id === item.id && activeItem?.type === activeTab
            }">
            <div class="z-10 w-full">
              <h2 class="truncate">{{ item.name }}</h2>
              <span class="opacity-50 text-sm">#{{ item.id }}</span>
            </div>
            <img v-if="item.thumb" :src="item.thumb + '&w=128&h=128'" alt="" class="absolute h-full aspect-square
                       object-cover right-0 top-0 mask-l-from-black opacity-50" />
          </li>
        </ul>
        <div ref="lastEl" />
      </div>
      <div v-if="isLoading && !error" class="p-4">
        <LoaderCircleIcon class="animate-spin mx-auto" :size="24" />
        Loading...
      </div>
    </ScrollArea>
  </aside>
</template>
