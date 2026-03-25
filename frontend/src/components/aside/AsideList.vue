<script setup lang="ts">
import {capitalize, concat, isArray} from "lodash-es";
import {computed, ref, shallowRef, triggerRef, watch} from "vue";
import {RouterLink, useRoute} from "vue-router";
import {vInfiniteScroll} from "@vueuse/components";
import type {WithRelations, Totalled} from "@/types";
import {parseFileMetaToUrl, useFileMetas, usePlatforms} from "@/lib/utils";
import AsideHeader from "./AsideHeader.vue";

const route = useRoute();

const apiUrl = computed(() => `/api/${route.name as string}${searchQuery.value}`);

const search = ref("");
const searchQuery = computed(() => search.value ? `?search=${encodeURIComponent(search.value)}` : "");

const page = ref(0);
const data = shallowRef<WithRelations<Totalled<any>> | null>(null);

const onLoadMore = async () => {

  if (!route.name) return

  const $data = data.value;
  const url = `${apiUrl.value}${searchQuery.value ? '&' : '?'}page=${page.value}`;
  const nextData = await (await fetch(url)).json();
  page.value += 1;

  triggerRef(data);

  if (!$data) return (data.value = nextData);

  // update data by concatenating new items to existing ones
  for (const key in nextData) {
    // @ts-expect-error - we know this is the correct type, but TS can't infer it
    if (isArray($data[key])) $data[key] = concat($data[key], nextData[key])
  }
}

const canLoadMore = () => {
  const $data = data.value;
  if (!$data) return false;
  return $data.total > page.value * 20; // can load if we haven't loaded all items yet
}

watch(apiUrl, () => {
  page.value = 0;
  data.value = null;
  onLoadMore();
}, {immediate: true})

// parse data to get required info for list items
function getHref(item: any): string {
  return `/${route.name as string}/${item.id}`;
}

function getTitle(item: any): string {
  return item.title ?? item.name ?? "Untitled";
}

const fileMetas = useFileMetas(data);
function getThumbnail(item: any): string | null {
  if (!item.thumb) return null
  const fileMeta = fileMetas.value[item.thumb];
  if (!fileMeta) return null
  return parseFileMetaToUrl(fileMeta)
}

const platforms = usePlatforms(data);
function getPlatform(item: any): string | null {
  if (!item.platform) return null
  const platform = platforms.value[item.platform];
  return platform ? platform.name : null;
}
</script>

<template>
  <AsideHeader v-model="search" />
  <ul class="h-full overflow-y-auto overflow-x-hidden" v-infinite-scroll="[onLoadMore, {canLoadMore}]">
    <li v-if="data" v-for="item in data.items" :key="item.id">
      <RouterLink :to="getHref(item)" class="block">
        <article class="text-sm pl-2 pr-1 py-1 hover:bg-secondary cursor-pointer flex gap-2">
          <div class="flex-1 min-w-0">
            <h3 class="font-bold truncate">
              {{ capitalize(getTitle(item)) }}
            </h3>
            <p class="text-xs text-muted-foreground truncate">
              #{{ item.id }}
              <span v-if="getPlatform(item)"> - {{ capitalize(getPlatform(item)!) }}</span>
            </p>
          </div>
          <img v-if="getThumbnail(item)" :src="getThumbnail(item)! + '?w=40&h=40'" alt="thumbnail"
                class="w-10 h-10 object-cover rounded flex-shrink-0" />
        </article>
      </RouterLink>
    </li>
    <div class="w-full h-px" />
  </ul>
</template>
