<script setup lang="ts">
import DrawerAside from '@/components/aside/DrawerAside.vue';
import AuthorsInput from '@/components/author/AuthorsInput.vue';
import CollectionsInput from '@/components/collection/CollectionsInput.vue';
import DateTimeInput from '@/components/input/DateTimeInput.vue';
import ModifyButtons from '@/components/input/ModifyButtons.vue';
import UrlInput from '@/components/input/UrlInput.vue';
import PlatformSelect from '@/components/platform/PlatformSelect.vue';
import {AsyncEditor} from '@/components/post/editor';
import PostFileList from '@/components/post/list/PostFileList.vue';
import TagsInput from '@/components/tag/TagsInput.vue';
import {Input} from '@/components/ui/input';
import {Tabs, TabsContent, TabsList, TabsTrigger} from '@/components/ui/tabs';
import {parseFileMetaToUrl, toastResponse, useChangeable, useDifference, useFileMetas, usePlatforms, useRemoveAPI} from '@/lib/utils';
import type {PostResponse, WithRelations} from '@/types';
import {useFetch} from '@vueuse/core';
import {filter, pick} from 'lodash-es';
import {computed} from 'vue';
import {useRoute} from 'vue-router';


const route = useRoute();
const id = computed(() => route.params.id)
const url = computed(() => `/api/posts/${id.value}`);

const {data: raw, execute} = useFetch(url, {refetch: true}).json<WithRelations<PostResponse>>();

const changed = useChangeable(raw)

const fileMetas = useFileMetas(changed)
const platforms = usePlatforms(changed)

const postFileMetas = computed(() => filter(fileMetas.value, fileMeta => fileMeta.post === changed.value?.id))

const difference = useDifference(raw, changed)
async function save() {
  const diffValues = pick(difference.value!, [
    "title",
    "source",
    "content",
    "thumb",
    "comments",
    "updated",
    "published",
    "platform",
  ]);

  const relationsKeys = ["authors", "tags", "collections"] as const;
  const relations = pick(difference.value!, relationsKeys);

  const diffRelations: Record<string, number[]> = {};
  for (const key of relationsKeys) {
    if (!relations[key]) continue;
    diffRelations[key] = relations[key].map((item: any) => item.id)
  }

  const diff = {
    ...diffValues,
    ...diffRelations,
  }

  console.log('Save post:', diff)
  if (
    await toastResponse(fetch(url.value, {
      method: "PATCH",
      body: JSON.stringify(diff),
      headers: {"Content-Type": "application/json"},
    }),
      "Post updated successfully",
      "Failed to update post"
    )
  ) raw.value = changed.value;
}

const remove = useRemoveAPI('post', url)
</script>

<template>
  <div v-if="changed" class="p-4 flex gap-4">
    <div class="flex-4 flex flex-col gap-4 pb-4">
      <AsyncEditor v-model="changed.content" :file-metas="fileMetas" v-memo="[raw?.id]" />
      <ModifyButtons class="max-md:mb-8" :disabled="!difference" @save="save" @delete="remove" />
    </div>

    <DrawerAside class="w-0">
      <Tabs default-value="details">
        <TabsList>
          <TabsTrigger value="details">
            Details
          </TabsTrigger>
          <TabsTrigger value="files">
            Files
          </TabsTrigger>
        </TabsList>
        <TabsContent value="details" class="flex flex-col gap-2">
          <img v-if="changed.thumb" :src="parseFileMetaToUrl(fileMetas[changed.thumb]!)" alt="cover"
            class="aspect-video object-cover rounded-md">

          <Input v-model="changed.title" class="text-2xl" />

          <div class="flex flex-col gap-2">
            Source:
            <UrlInput v-model="changed.source" />
          </div>


          <div class="flex flex-col gap-2">
            Platform
            <PlatformSelect nullable v-model="changed.platform" :platforms="platforms" />
          </div>

          <div class="flex flex-col gap-2">
            Published:
            <DateTimeInput v-model="changed.published" label="Published" />
            Updated:
            <DateTimeInput v-model="changed.updated" label="Updated" />
          </div>

          <div class="flex flex-col gap-2">
            Authors:
            <AuthorsInput v-model="changed.authors" :file-metas="fileMetas" />
          </div>

          <div class="flex flex-col gap-2">
            Tags:
            <TagsInput v-model="changed.tags" :platforms="platforms" />
          </div>

          <div class="flex flex-col gap-2">
            Collections:
            <CollectionsInput v-model="changed.collections" :file-metas="fileMetas" />
          </div>

        </TabsContent>
        <TabsContent value="files">

          <PostFileList v-model:thumb="changed.thumb" :post="changed.id" :fileMetas="postFileMetas" @refresh="execute"
            class="sticky top-0" />

        </TabsContent>
      </Tabs>
    </DrawerAside>
  </div>
</template>
