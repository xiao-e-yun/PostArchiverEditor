<script setup lang="ts">
import {commitRef, reactiveChanges, useActiveItem, useRelations} from '@/utils';
import type {WithRelations} from '@api/WithRelations';
import {computed, ref, toRef} from 'vue';
import {Input} from '../ui/input';
import {isEmpty} from 'lodash-es';
import {Button} from '../ui/button';
import {ButtonGroup} from '../ui/button-group';
import type {FileMeta} from 'post-archiver';
import CategoryInput from '../inputs/CategoryInput.vue';
import {CategoryType} from '@/category';
import {Textarea} from '../ui/textarea';

const props = defineProps<{
  data: WithRelations<FileMeta>
}>();

const data = toRef(props, 'data');
const proxyed = ref(reactiveChanges(data.value));
const relations = computed(() => useRelations(props.data));

const mime = computed({
  get: () => (proxyed.value.mime.split('/') || ['', '']) as [string, string],
  set: (val: [string, string]) => proxyed.value.mime = val.join('/')
})

const extra = computed({
  get: () => JSON.stringify(proxyed.value.extra, null, 2),
  set: (val: string) => {
    extraIsValid.value = true
    try {
      proxyed.value.extra = JSON.parse(val);
    } catch {
      console.warn('Invalid JSON for extra data');
      extraIsValid.value = false
    }
  }
})
const extraIsValid = ref(true);

const updateFileMeta = () => {
  if (isEmpty(proxyed.value.changes)) return;
  const id = data.value.id;
  fetch(`/api/file_metas/${id}`, {
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
      alert('FileMeta updated successfully');
    } else {
      const error = await res.text();
      alert(`Error updating file_meta: ${error} ${res.statusText}`);
    }
  }).catch((err) => {
    alert(`Error updating file_meta: ${err.message}`);
  });
}

const deleteFileMeta = () => {
  if (!confirm('Are you sure you want to delete this file_meta?')) return
  const id = data.value.id;
  fetch(`/api/file_metas/${id}`, {
    method: 'DELETE',
  }).then(async (res) => {
    if (res.ok) {
      alert('FileMeta deleted successfully');
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
  <div class="flex flex-col gap-4 w-full mx-auto lg:w-lg">
    <Input v-model="proxyed.filename" class="w-full h-max p-2 border text-2xl!" placeholder="Title" />
    <div>
      <span class="ml-2">Mime:</span>
      <div class="rounded-md bg-input px-3 py-1 text-sm shadow-sm cursor-pointer mt-1 flex
        items-center gap-3 border border-transparent">
        <Input v-model="mime[0]" class="w-1/2 bg-transparent! p-0 m-0 border-0 focus:ring-0 text-center
           focus:outline-0" placeholder="type" />
        /
        <Input v-model="mime[1]" class="w-1/2 bg-transparent! p-0 m-0 border-0 focus:ring-0 text-center
           focus:outline-0" placeholder="subtype" />
      </div>
    </div>

    <CategoryInput v-model="proxyed.post" :type="CategoryType.Post" :relations="relations" />

    <div class="relative w-full">
      <span class="ml-2">Extra Data:</span>
      <Textarea v-model.lazy="extra"
        class="w-full h-32 p-2 border rounded-md bg-input text-sm shadow-sm mt-1 resize-none focus:ring-0 focus:outline-0"
        placeholder="Extra metadata in JSON format" />
      <span v-if="!extraIsValid" class="absolute top-2 right-2 text-sm text-red-500">
        Invalid JSON
      </span>
    </div>

    <div class="flex mt-4 justify-between">
      <Button variant="destructive" @click="deleteFileMeta">Delete</Button>
      <ButtonGroup v-if="!isEmpty(proxyed.changes)" class="flex gap-2 justify-end">
        <Button variant="destructive" @click="proxyed.changes = {}">Discard</Button>
        <Button @click="updateFileMeta">Save</Button>
      </ButtonGroup>
    </div>
  </div>
</template>
