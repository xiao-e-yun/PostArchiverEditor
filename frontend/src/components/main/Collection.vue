<script setup lang="ts">
import {reactiveChanges, useActiveItem, useRelations} from '@/utils';
import type {WithRelations} from '@api/WithRelations';
import {computed, ref, toRef} from 'vue';
import {Input} from '../ui/input';
import {isEmpty} from 'lodash-es';
import {CategoryType} from '@/category';
import CategoryInput from '../inputs/CategoryInput.vue';
import ActionButtons from '../inputs/ActionButtons.vue';
import type {Collection} from 'post-archiver';
import ImageInput from '../inputs/ImageInput.vue';

const props = defineProps<{
  data: WithRelations<Collection>
}>();

const data = toRef(props, 'data');
const proxyed = ref(reactiveChanges(data.value));

const proxyedSource = computed({
  get: () => proxyed.value.source || '',
  set: (val: string) => proxyed.value.source = isEmpty(val) ? null : val
})

const updateTag = () => {
  if (isEmpty(proxyed.value.changes)) return;
  const id = data.value.id;
  fetch(`/api/collections/${id}`, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(proxyed.value.changes)
  }).then(async (res) => {
    if (res.ok) {
      const updatedPost = await res.json();
      Object.assign(data.value, updatedPost);
      proxyed.value = reactiveChanges(data.value);
      alert('Collection updated successfully');
    } else {
      const error = await res.text();
      alert(`Error updating collection: ${error} ${res.statusText}`);
    }
  }).catch((err) => {
    alert(`Error updating collection: ${err.message}`);
  });
}

const deletePost = () => {
  if (!confirm('Are you sure you want to delete this collection?')) return
  const id = data.value.id;
  fetch(`/api/collections/${id}`, {
    method: 'DELETE',
  }).then(async (res) => {
    if (res.ok) {
      alert('Collection deleted successfully');
      useActiveItem().value = null;
    } else {
      const error = await res.text();
      alert(`Error deleting collection: ${error}`);
    }
  }).catch((err) => {
    alert(`Error deleting collection: ${err.message}`);
  });
}
</script>

<template>
  <div class="flex flex-col gap-4 w-full mx-auto lg:w-lg">
    <Input v-model="proxyed.name" class="w-full h-max p-2 border text-2xl!" placeholder="Title" />
    <Input v-model="proxyedSource" class="w-full" placeholder="Source URL" />
    <ActionButtons
      :changes="proxyed.changes"
      @save="updateTag"
      @discard="proxyed.changes = {}"
      @delete="deletePost"
    />
  </div>
</template>
