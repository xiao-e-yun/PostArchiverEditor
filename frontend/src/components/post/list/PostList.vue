<script setup lang="ts">
import {concat, isArray} from "lodash-es";
import {computed, ref, shallowRef, triggerRef, watch} from "vue";
import {RouterLink} from "vue-router";
import {vInfiniteScroll} from "@vueuse/components";
import type {WithRelations, Totalled, PostShortResponse} from "@/types";
import {parseFileMetaToUrl, useFileMetas, usePlatforms} from "@/lib/utils";
import {ChevronUp} from "lucide-vue-next";

const props = defineProps<{
  author?: number,
  collection?: number,
  tag?: number
  platform?: number
}>()

const hidden = ref(false);

const apiUrl = computed(() => {
  const url = `/api/posts`
  return url + query.value;
})

const query = computed(() => {
  const params = new URLSearchParams();
  if (props.author !== undefined) params.append("author", props.author.toString());
  if (props.collection !== undefined) params.append("collection", props.collection.toString());
  if (props.tag !== undefined) params.append("tag", props.tag.toString());
  if (props.platform !== undefined) params.append("platform", props.platform.toString());
  return params.size ? `?${params.toString()}` : "";
})

const page = ref(0);
const data = shallowRef<WithRelations<Totalled<PostShortResponse>> | null>(null);

const onLoadMore = async () => {
  const $data = data.value;
  const url = `${apiUrl.value}${query.value ? '&' : '?'}page=${page.value}`;
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

const fileMetas = useFileMetas(data);
function getThumbnail(item: PostShortResponse): string | null {
  if (!item.thumb) return null
  const fileMeta = fileMetas.value[item.thumb];
  if (!fileMeta) return null
  return parseFileMetaToUrl(fileMeta)
}

const platforms = usePlatforms(data);
function getPlatform(item: PostShortResponse): string | null {
  if (!item.platform) return null
  const platform = platforms.value[item.platform];
  return platform ? platform.name : null;
}
</script>

<template>
  <div class="w-full border rounded-md">
    <div class="px-2 py-1 font-bold flex items-center justify-between">
      <div>
        Posts
        <span v-if="data" class="text-sm text-muted-foreground">
          ({{ data.total }})
        </span>
      </div>
      <ChevronUp :class="{'mirror': !hidden}" class="hover:bg-secondary rounded [&.mirror]:scale-y-[-1] cursor-pointer" @click="hidden = !hidden" />
    </div>
    <ul v-if="data && !hidden" class="w-full h-64 max-h-128 overflow-y-auto
                               overflow-x-hidden border-t"
      v-infinite-scroll="[onLoadMore, {canLoadMore}]">
      <li v-for="item in data.items" :key="item.id">
        <RouterLink :to="'/posts/' + item.id" class="block">
          <article class="text-sm pl-2 pr-1 py-1 hover:bg-secondary cursor-pointer flex gap-2">
            <div class="flex-1 overflow-hidden">
              <h3 class="font-bold truncate">
                {{ item.title }}
              </h3>
              <p class="text-xs text-foreground truncate">
                #{{ item.id }}
                <span v-if="getPlatform(item)"> - {{ getPlatform(item) }}</span>
              </p>
            </div>
            <img v-if="getThumbnail(item)" :src="getThumbnail(item)! + '?w=40&h=40'" alt="thumbnail"
              class="w-10 h-10 object-cover rounded flex-shrink-0" />
          </article>
        </RouterLink>
      </li>
    </ul>
  </div>
</template>
