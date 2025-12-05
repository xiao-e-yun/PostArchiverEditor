<script setup lang="ts">
import {reactiveChanges, useActiveItem, useRelations} from '@/utils';
import type {PostResponse} from '@api/PostResponse';
import type {WithRelations} from '@api/WithRelations';
import {computed, ref, toRef} from 'vue';
import ImageInput from '../inputs/ImageInput.vue';
import {Input} from '../ui/input';
import DateTimeInput from '../inputs/DateTimeInput.vue';
import {isEmpty} from 'lodash-es';
import ContentInput from '../inputs/ContentInput.vue';
import CategoryGroupInput from '../inputs/CategoryGroupInput.vue';
import { CategoryType } from '@/category';
import CategoryInput from '../inputs/CategoryInput.vue';
import ActionButtons from '../inputs/ActionButtons.vue';

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

const updatePost = () => {
  if (isEmpty(proxyed.value.changes)) return;
  const id = data.value.id;
  fetch(`/api/posts/${id}`, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      ...proxyed.value.changes,
      authors: proxyed.value.changes.authors?.map(a => a.id),
      tags:  proxyed.value.changes.tags?.map(t => t.id),
      collections:  proxyed.value.changes.collections?.map(c => c.id),
    })
  }).then(async (res) => {
    if (res.ok) {
      const updatedPost = await res.json();
      Object.assign(data.value, updatedPost);
      proxyed.value = reactiveChanges(data.value);
      alert('Post updated successfully');
    } else {
      const error = await res.text();
      alert(`Error updating post: ${error} ${res.statusText}`);
    }
  }).catch((err) => {
    alert(`Error updating post: ${err.message}`);
  });
}

const deletePost = () => {
  if (!confirm('Are you sure you want to delete this post?')) return
  const id = data.value.id;
  fetch(`/api/posts/${id}`, {
    method: 'DELETE',
  }).then(async (res) => {
    if (res.ok) {
      alert('Post deleted successfully');
      useActiveItem().value = null;
    } else {
      const error = await res.text();
      alert(`Error deleting post: ${error}`);
    }
  }).catch((err) => {
    alert(`Error deleting post: ${err.message}`);
  });
}
</script>

<template>
  <div class="flex gap-4 max-lg:flex-col-reverse relative">
    <div class="flex flex-col gap-4 w-full">
      <Input v-model="proxyed.title" class="max-lg:hidden w-full h-max p-2 border text-2xl!" placeholder="Title" />
      <ContentInput v-model="proxyed.content" :post="data.id" :file-metas="relations.file_metas" class="w-full" />
      <ActionButtons
        :changes="proxyed.changes"
        @save="updatePost"
        @discard="proxyed.changes = {}"
        @delete="deletePost"
      />
    </div>
    <div class="flex flex-col gap-4 lg:w-lg lg:sticky lg:top-0 lg:self-start">
      <ImageInput v-model="proxyed.thumb" :post="data.id" :file-metas="relations.file_metas" accepts="image" />
      <Input v-model="proxyed.title" class="lg:hidden w-full h-max p-2 border text-2xl!" placeholder="Title" />
      <Input v-model="proxyedSource" class="w-full" placeholder="Source URL" />
      <div>
        <span class="ml-2">Published:</span>
        <DateTimeInput v-model="proxyed.published" />
      </div>
      <div>
        <span class="ml-2">Updated:</span>
        <DateTimeInput v-model="proxyed.updated" />
      </div>
      <CategoryGroupInput
        v-model="proxyed.authors"
        :type="CategoryType.Author"
        :relations="relations"
      />
      <CategoryGroupInput
        v-model="proxyed.tags"
        :type="CategoryType.Tag"
        :relations="relations"
      />
      <CategoryGroupInput
        v-model="proxyed.collections"
        :type="CategoryType.Collection"
        :relations="relations"
      />
      <CategoryInput
        v-model="proxyed.platform"
        :type="CategoryType.Platform"
        :relations="relations"
      />
    </div>
  </div>
</template>
